pub mod card {
    #[derive(Debug)]
    pub struct BingoCard {
        b: Vec<i32>,
        i: Vec<i32>,
        n: Vec<i32>,
        g: Vec<i32>,
        o: Vec<i32>,
        dabbed: Vec<i32>,
    }

    impl BingoCard {
        pub fn new(rows: Vec<Vec<i32>>) -> BingoCard {
            BingoCard {
                b: rows[0].clone(),
                i: rows[1].clone(),
                n: rows[2].clone(),
                g: rows[3].clone(),
                o: rows[4].clone(),
                dabbed: Vec::new(),
            }
        }

        pub fn empty() -> BingoCard {
            BingoCard {
                b: Vec::new(),
                i: Vec::new(),
                n: Vec::new(),
                g: Vec::new(),
                o: Vec::new(),
                dabbed: Vec::new(),
            }
        }

        #[cfg(test)]
        pub fn test_draws() -> Vec<i32> {
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ]
        }

        #[cfg(test)]
        pub fn test_cards() -> Vec<BingoCard> {
            vec![
                BingoCard::new(vec![
                    vec![22, 13, 17, 11, 0],
                    vec![8, 2, 23, 4, 24],
                    vec![21, 9, 14, 16, 7],
                    vec![6, 10, 3, 18, 5],
                    vec![1, 12, 20, 15, 19],
                ]),
                BingoCard::new(vec![
                    vec![3, 15, 0, 2, 22],
                    vec![9, 18, 13, 17, 5],
                    vec![19, 8, 7, 25, 23],
                    vec![20, 11, 10, 24, 4],
                    vec![14, 21, 16, 12, 6],
                ]),
                BingoCard::new(vec![
                    vec![14, 21, 17, 24, 4],
                    vec![10, 16, 15, 9, 19],
                    vec![18, 8, 23, 26, 20],
                    vec![22, 11, 13, 6, 5],
                    vec![2, 0, 12, 3, 7],
                ]),
            ]
        }

        pub fn clone(&mut self) -> BingoCard {
            BingoCard {
                b: self.b.clone(),
                i: self.i.clone(),
                n: self.n.clone(),
                g: self.g.clone(),
                o: self.o.clone(),
                dabbed: self.dabbed.clone(),
            }
        }

        pub fn dab(&mut self, n: i32) {
            self.dabbed.push(n);
        }

        pub fn check_rows(&mut self) -> bool {
            let mut winners: Vec<bool> = Vec::new();
            winners.push(self.b.iter().all(|x| self.dabbed.contains(x)));
            winners.push(self.i.iter().all(|x| self.dabbed.contains(x)));
            winners.push(self.n.iter().all(|x| self.dabbed.contains(x)));
            winners.push(self.g.iter().all(|x| self.dabbed.contains(x)));
            winners.push(self.o.iter().all(|x| self.dabbed.contains(x)));

            winners.iter().any(|x| *x)
        }

        pub fn get_column(&self, col: usize) -> Vec<i32> {
            vec![
                self.b[col],
                self.i[col],
                self.n[col],
                self.g[col],
                self.o[col],
            ]
        }

        pub fn check_cols(&mut self) -> bool {
            let mut winners: Vec<bool> = Vec::new();
            for i in 0..5 {
                winners.push(self.get_column(i).iter().all(|x| self.dabbed.contains(x)));
            }

            winners.iter().any(|x| *x)
        }

        pub fn is_winner(&mut self) -> bool {
            self.check_cols() | self.check_rows()
        }

        pub fn solve(&mut self) -> i32 {
            let mut vec = Vec::new();
            vec.append(&mut self.b);
            vec.append(&mut self.i);
            vec.append(&mut self.n);
            vec.append(&mut self.g);
            vec.append(&mut self.o);
            let sum: i32 = vec.iter().filter(|x| !self.dabbed.contains(x)).sum();

            let last_draw: i32 = self.dabbed.pop().unwrap();

            sum * last_draw
        }
    }
}
