pub fn tower_builder(n_floors: usize, block_size: (usize, usize)) -> Vec<String> {
    let (width, height) = block_size;
    let mut result = Vec::new();
    
    // Максимальная ширина башни (нижний этаж)
    let max_width = (2 * n_floors - 1) * width;
    
    for floor in (0..n_floors).rev() {
        // Количество блоков на текущем этаже
        let blocks = 2 * floor + 1;
        // Ширина текущего этажа в символах
        let current_width = blocks * width;
        // Количество пробелов с каждой стороны
        let spaces = (max_width - current_width) / 2;
        
        // Создаем строку для каждого "слоя" блока по высоте
        let floor_str = format!(
            "{}{}{}",
            " ".repeat(spaces),
            "*".repeat(current_width),
            " ".repeat(spaces)
        );
        
        // Добавляем нужное количество копий строки (по высоте блока)
        for _ in 0..height {
            result.push(floor_str.clone());
        }
    }
    
    // Разворачиваем, так как мы добавляли с верхнего этажа
    result.reverse();

    result
}