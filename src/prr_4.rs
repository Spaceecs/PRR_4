#[test]
fn prr_4test() {
    const SIZE: usize = 6; // Розмір ромба (кількість зірочок у найширшій частині)

    // Верхня частина ромба
    for i in 0..SIZE {
        let stars = 2 * i + 1; // Кількість зірочок
        let spaces = SIZE - i - 1; // Кількість пробілів
        print!("{}{}", " ".repeat(spaces), "*".repeat(stars)); // Друкуємо пробіли та зірочки
        println!(); // Перехід на новий рядок
    }

    // Нижня частина ромба
    for i in (0..SIZE-1).rev() {
        let stars = 2 * i + 1; // Кількість зірочок
        let spaces = SIZE - i - 1; // Кількість пробілів
        print!("{}{}", " ".repeat(spaces), "*".repeat(stars)); // Друкуємо пробіли та зірочки
        println!(); // Перехід на новий рядок
    }
}