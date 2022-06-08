

fn gen(input: &str) -> Vec<usize> {
    input.trim().split(' ').map(|x| x.trim().parse().unwrap()).collect()
}
struct Item {
    children : Vec<Item>,
    metadata : Vec<usize>
}
impl Item {
    fn meta_local(&self) -> usize {
        self.metadata.iter().sum()
    }
    fn meta_total(&self) -> usize {
        self.meta_local() + self.children.iter().map(|c| c.meta_total()).sum::<usize>()
    }
    fn value(&self) -> usize {
        if self.children.is_empty() {self.meta_local()}
        else {
            self.metadata.iter().map(|m| self.children.get(m-1).map(|c| c.value()).unwrap_or(0)).sum()
        }
    }
}

fn parse_item(input: &mut dyn Iterator<Item = usize>) -> Item {
    let children_count = input.next().expect("Child count");
    let meta_count = input.next().expect("meta count");
    Item {
        children: (0..children_count).map(|_| parse_item(input)).collect(),
        metadata: (0..meta_count).map(|_| input.next().expect("meta item")).collect()
    }
}

fn p1(input: &[usize]) -> usize {
    let mut iter = input.iter().cloned();
    let i = parse_item(&mut iter);
    i.meta_total()
}

fn p2(input: &[usize]) -> usize {
    let mut iter = input.iter().cloned();
    let i = parse_item(&mut iter);
    i.value()
}