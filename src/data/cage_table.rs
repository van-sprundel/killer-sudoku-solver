pub struct CageTable(pub Vec<TableItem>);

impl CageTable {
    pub fn from_file(str: &str) -> Self {
        let file = std::fs::read_to_string(str).unwrap();
        let lines = file.lines();

        let mut items = vec![];

        let mut cage_size = 0;
        lines.for_each(|line| if line.ends_with("cells") {
            cage_size += 1;
        } else if line.contains(':') {
            let mut combinations = vec![];
            let mut split = line.split_whitespace().collect::<Vec<_>>();

            let mut temp_sum = split.remove(0).to_string();
            temp_sum.pop();
            let cage_sum = temp_sum.parse::<i32>().unwrap();
            // println!("cage size {} {}", cage_size, cage_sum);
            for possibilities in split {
                let possibilities = possibilities
                    .chars()
                    .filter(|x| x.is_digit(10))
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>();
                combinations.push(possibilities);
            }
            // println!("{:?}", combinations);
            items.push(TableItem {
                cage_sum,
                cage_size,
                combinations,
            })
        });
        Self(items)
    }

    pub fn find(&mut self, size: i32, sum: i32) -> Option<TableItem> {
        self.0.iter().find(|ti| ti.cage_sum == sum && ti.cage_size == size).cloned()
    }
}

#[derive(Debug, Clone)]
pub struct TableItem {
    pub cage_size: i32,
    pub cage_sum: i32,
    pub combinations: Vec<Vec<u32>>,
}