use std::time::{UNIX_EPOCH, SystemTime, Duration};
use std::thread::sleep;
use std::io::{stdout, Write};

#[link(name = "c")]
extern "C" {
    fn rand() -> i16;
    fn srand(seed: u32);
}

#[allow(non_upper_case_globals)]
const flush: fn() = || stdout().flush().unwrap();
#[allow(non_upper_case_globals)]
const clear: fn() = || print!("\x1b[1;1H\x1b[2J");

fn display_list(list: &[i16]) {
    let mut max = 0;

    let len = list.len();

    for i in 0..len {
        if list[i] > max {
            max = list[i];
        }
    }

    for i in 0..max {
        for j in 0..len {
            if list[j] >= max - i {
                print!("| ");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}

fn insertive_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);
    let len = list.len();

    for i in 1..len {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j, j - 1);
            j -= 1;
            swaps += 1;
        }
        comparisons += 1;
        clear();
        flush();
        println!("Insertive sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
        display_list(&list);
        sleep(Duration::from_millis(40));
    }
}

fn gnome_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);
    let len = list.len();

    let mut i = 1;
    let mut j = 2;

    while i < len {
        if list[i - 1] <= list[i] {
            i = j;
            j += 1;
        } else {
            list.swap(i - 1, i);
            i -= 1;
            swaps += 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
        comparisons += 1;
        clear();
        flush();
        println!("Gnome sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
        display_list(&list);
        sleep(Duration::from_millis(40));
    }
}

fn cocktail_shaker_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);
    let len = list.len();

    let mut swapped = true;
    let mut start = 0;
    let mut end = len - 1;

    while swapped {
        swapped = false;

        for i in start..end {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                clear();
                flush();
                println!("Cocktail shaker sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
                display_list(&list);
                sleep(Duration::from_millis(20));
                swapped = true;
                swaps += 1;
            }
            comparisons += 1;
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        for i in (start..end).rev() {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                clear();
                flush();
                println!("Cocktail shaker sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
                display_list(&list);
                swapped = true;
                swaps += 1;
            }
            comparisons += 1;
        }

        start += 1;
        sleep(Duration::from_millis(20));
    }
}

fn shell_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);
    let len = list.len();

    let mut gap = len / 2;

    while gap > 0 {
        for i in gap..len {
            let temp = list[i];
            let mut j = i;
            while j >= gap && list[j - gap] > temp {
                list[j] = list[j - gap];
                j -= gap;
                swaps += 1;
                clear();
                flush();
                println!("Shell sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
                display_list(&list);
                sleep(Duration::from_millis(40));
            }
            list[j] = temp;
            comparisons += 1;
        }
        gap /= 2;
    }
    clear();
    flush();
    println!("Shell sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
    display_list(&list);
    sleep(Duration::from_millis(40));
}

fn comb_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);
    let len = list.len();

    let mut gap = len;
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink) as usize;

        if gap > 1 {
            sorted = false;
        } else {
            gap = 1;
            sorted = true;
        }

        let mut i = 0;
        while i + gap < len {
            if list[i] > list[i + gap] {
                list.swap(i, i + gap);
                sorted = false;
                swaps += 1;
                clear();
                flush();
                println!("Comb sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
                display_list(&list);
                sleep(Duration::from_millis(40));
            }
            comparisons += 1;
            i += 1;
        }
    }
    clear();
    flush();
    println!("Comb sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
    display_list(&list);
    sleep(Duration::from_millis(40));
}

fn selection_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);

    let len = list.len();

    for i in 0..len {
        let mut min = i;
        for j in i..70 {
            if list[j] < list[min] {
                min = j;
            }
            comparisons += 1;
        }
        list.swap(i, min);
        swaps += 1;
        clear();
        flush();
        println!("Selective sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
        display_list(&list);
        sleep(Duration::from_millis(40));
    }
}

fn bubble_sort(list: &mut [i16]) {
    let (mut comparisons, mut swaps) = (0, 0);
    let len = list.len();

    for _ in 0..len {
        for j in 0..len - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                swaps += 1;
            }
            comparisons += 1;
        }
        clear();
        flush();
        println!("Bubble sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
        display_list(&list);
        sleep(Duration::from_millis(40));
    }
}

fn quick_sort(list: &mut [i16], low: isize, high: isize, sw: &mut usize, comp: &mut usize) {
    if low < high {
        *comp += 1;
        let pivot = partition(list, low, high, sw, comp);
        quick_sort(list, low, pivot - 1, sw, comp);
        quick_sort(list, pivot + 1, high, sw, comp);
    }
}

fn partition(list: &mut [i16], low: isize, high: isize, sw: &mut usize, comp: &mut usize) -> isize {
    let pivot = list[high as usize];
    let mut i = low - 1;

    for j in low..=(high - 1) {
        if list[j as usize] < pivot {
            *sw += 1;
            i += 1;
            list.swap(i as usize, j as usize);
            clear();
            flush();
            println!("Quick sort... (comparisons: {}) (swaps: {})\n", comp, sw);
            display_list(&list);
            sleep(Duration::from_millis(10));
        }
        *comp += 1;
    }
    list.swap((i+1) as usize, high as usize);
    *sw += 1;
    clear();
    flush();
    println!("Quick sort... (comparisons: {}) (swaps: {})\n", comp, sw);
    display_list(&list);
    sleep(Duration::from_millis(20));

    i + 1
}

fn bogo_sort(list: &mut [i16]) {
    let mut sorted = false;
    let mut comparisons = 0;
    let mut swaps = 0;
    let len = list.len();

    while !sorted {
        for _ in 0..len {
            unsafe { list.swap(rand() as usize % len, rand() as usize % len); }
            swaps += 1;
        }
        for i in 0..69 {
            if list[i] > list[i + 1] {
                sorted = false;
                break;
            } else {
                sorted = true;
            }
            comparisons += 1;
        }
        clear();
        flush();
        println!("Bogo sort...  (comparisons: {}) (swaps: {})\n", comparisons, swaps);
        display_list(&list);
        sleep(Duration::from_millis(20));
    }
}

fn assert_sorted(list: &[i16]) {
    let len = list.len() - 1;
    for i in 0..len {
        print!("\r{}^\n{}|", "  ".repeat(list[i] as usize), "  ".repeat(list[i] as usize));
        flush();
        sleep(Duration::from_millis(20));
        if list[i] > list[i + 1] {
            panic!("\nList is not sorted!");
        }
        //move cursor up and to the right
        print!("\x1b[1A\x1b[2C");
    }
    flush();
    print!("\r{}^\n{}|\n", "  ".repeat(len), "  ".repeat(len));
    println!("List is sorted!");
}

fn main() -> std::io::Result<()> {
    const LIST_LEN: usize = 70;
    let mut list: [i16; LIST_LEN] = [0; LIST_LEN];
    for i in 0..LIST_LEN {
        list[i] = i as i16;
    }
    display_list(&list);
    unsafe {
        srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos());
    }

    // Shuffle the list
    for _ in 0..140 {
        unsafe { list.swap(rand() as usize % LIST_LEN, rand() as usize % LIST_LEN); }
        clear();
        flush();
        println!("Shuffling... \n");
        display_list(&list);
        sleep(Duration::from_millis(20));
    }

    print!("1.) Selection Sort\n2.) Bubble Sort\n3.) Quick Sort\n4.) Bogo Sort ( ͡° ͜ʖ ͡°)\n5.) Insertive Sort\n6.) Gnome Sort\n7.) Cocktail Shaker Sort\n8.) Comb Sort\n9.) Shell Sort\n\nEnter a number: ");
    flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<u8>().unwrap() {
        1 => {
            selection_sort(&mut list);
            assert_sorted(&list);
        },
        2 => {
            bubble_sort(&mut list);
            assert_sorted(&list);
        },
        3 => {
            let len = list.len();
            let (mut swaps, mut comparisons) = (0, 0);
            quick_sort(&mut list, 0, (len - 1) as isize, &mut swaps, &mut comparisons);
            assert_sorted(&list);
        },
        4 => {
            bogo_sort(&mut list);
            assert_sorted(&list);
        },
        5 => {
            insertive_sort(&mut list);
            assert_sorted(&list);
        },
        6 => {
            gnome_sort(&mut list);
            assert_sorted(&list);
        },
        7 => {
            cocktail_shaker_sort(&mut list);
            assert_sorted(&list);
        },
        8 => {
            comb_sort(&mut list);
            assert_sorted(&list);
        },
        9 => {
            shell_sort(&mut list);
            assert_sorted(&list);
        },
        _ => {
            println!("Invalid input!");
        }
    }

    Ok(())
}
