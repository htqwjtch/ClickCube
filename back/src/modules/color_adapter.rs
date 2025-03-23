use std::sync::{OnceLock, RwLock};

pub struct ColorAdapter;

static RAW_COLORS: OnceLock<RwLock<Vec<Vec<String>>>> = OnceLock::new();
static ADAPTED_COLORS: OnceLock<RwLock<Vec<Vec<String>>>> = OnceLock::new();

impl ColorAdapter {
    pub fn set_raw_colors(raw_colors: Vec<Vec<String>>) {
        let mut colors = Vec::new();
        for k in 0..6 {
            let color = ColorAdapter::adapt_color(raw_colors.clone(), k);
            colors.push(color);
        }

        let lock = RAW_COLORS.get_or_init(|| RwLock::new(vec![]));
        *lock.write().unwrap() = raw_colors; // replace data, not clear
    }

    pub fn get_raw_colors() -> Vec<Vec<String>> {
        let lock = RAW_COLORS.get().expect("Data not set");
        let data = lock.read().unwrap();
        data.clone() // return data copy, not move
    }

    pub fn receive_raw_colors(raw_colors: Vec<Vec<String>>) {
        let mut colors = Vec::new();
        for k in 0..6 {
            let color = ColorAdapter::adapt_color(raw_colors.clone(), k);
            colors.push(color);
        }

        let lock = ADAPTED_COLORS.get_or_init(|| RwLock::new(vec![]));
        *lock.write().unwrap() = colors; // replace data, not clear
    }

    //TO OPTIMIZE!!!
    fn adapt_color(colors: Vec<Vec<String>>, num: usize) -> Vec<String> {
        return match num {
            0 => {
                vec![
                    colors[0][0].clone() + colors[4][2].as_str() + colors[2][6].as_str(),
                    colors[0][1].clone() + colors[2][7].as_str(),
                    colors[0][2].clone() + colors[2][8].as_str() + colors[5][0].as_str(),
                    colors[0][3].clone() + colors[4][5].as_str(),
                    colors[0][4].clone(),
                    colors[0][5].clone() + colors[5][3].as_str(),
                    colors[0][6].clone() + colors[3][0].as_str() + colors[4][8].as_str(),
                    colors[0][7].clone() + colors[3][1].as_str(),
                    colors[0][8].clone() + colors[5][6].as_str() + colors[3][2].as_str(),
                ]
            }
            1 => {
                vec![
                    colors[1][0].clone() + colors[5][2].as_str() + colors[2][2].as_str(),
                    colors[1][1].clone() + colors[2][1].as_str(),
                    colors[1][2].clone() + colors[2][0].as_str() + colors[4][0].as_str(),
                    colors[1][3].clone() + colors[5][5].as_str(),
                    colors[1][4].clone(),
                    colors[1][5].clone() + colors[4][3].as_str(),
                    colors[1][6].clone() + colors[3][8].as_str() + colors[5][8].as_str(),
                    colors[1][7].clone() + colors[3][7].as_str(),
                    colors[1][8].clone() + colors[4][6].as_str() + colors[3][6].as_str(),
                ]
            }
            2 => {
                vec![
                    colors[2][0].clone() + colors[4][0].as_str() + colors[1][2].as_str(),
                    colors[2][1].clone() + colors[1][1].as_str(),
                    colors[2][2].clone() + colors[1][0].as_str() + colors[5][2].as_str(),
                    colors[2][3].clone() + colors[4][1].as_str(),
                    colors[2][4].clone(),
                    colors[2][5].clone() + colors[5][1].as_str(),
                    colors[2][6].clone() + colors[0][0].as_str() + colors[4][2].as_str(),
                    colors[2][7].clone() + colors[0][1].as_str(),
                    colors[2][8].clone() + colors[5][0].as_str() + colors[0][2].as_str(),
                ]
            }
            3 => {
                vec![
                    colors[3][0].clone() + colors[4][8].as_str() + colors[0][6].as_str(),
                    colors[3][1].clone() + colors[0][7].as_str(),
                    colors[3][2].clone() + colors[0][8].as_str() + colors[5][6].as_str(),
                    colors[3][3].clone() + colors[4][7].as_str(),
                    colors[3][4].clone(),
                    colors[3][5].clone() + colors[5][7].as_str(),
                    colors[3][6].clone() + colors[1][8].as_str() + colors[4][6].as_str(),
                    colors[3][7].clone() + colors[1][7].as_str(),
                    colors[3][8].clone() + colors[5][8].as_str() + colors[1][6].as_str(),
                ]
            }
            4 => {
                vec![
                    colors[4][0].clone() + colors[1][2].as_str() + colors[2][0].as_str(),
                    colors[4][1].clone() + colors[2][3].as_str(),
                    colors[4][2].clone() + colors[2][6].as_str() + colors[0][0].as_str(),
                    colors[4][3].clone() + colors[1][5].as_str(),
                    colors[4][4].clone(),
                    colors[4][5].clone() + colors[0][3].as_str(),
                    colors[4][6].clone() + colors[3][6].as_str() + colors[1][8].as_str(),
                    colors[4][7].clone() + colors[3][3].as_str(),
                    colors[4][8].clone() + colors[0][6].as_str() + colors[3][0].as_str(),
                ]
            }
            5 => {
                vec![
                    colors[5][0].clone() + colors[0][2].as_str() + colors[2][8].as_str(),
                    colors[5][1].clone() + colors[2][5].as_str(),
                    colors[5][2].clone() + colors[2][2].as_str() + colors[1][0].as_str(),
                    colors[5][3].clone() + colors[0][5].as_str(),
                    colors[5][4].clone(),
                    colors[5][5].clone() + colors[1][3].as_str(),
                    colors[5][6].clone() + colors[3][2].as_str() + colors[0][8].as_str(),
                    colors[5][7].clone() + colors[3][5].as_str(),
                    colors[5][8].clone() + colors[1][6].as_str() + colors[3][8].as_str(),
                ]
            }
            _ => panic!("Invalid face number: {}", num),
        };
    }

    pub fn transmit_adapted_colors() -> Vec<Vec<String>> {
        ADAPTED_COLORS
            .get()
            .map_or_else(|| vec![], |lock| lock.read().unwrap().clone())
    }
}
