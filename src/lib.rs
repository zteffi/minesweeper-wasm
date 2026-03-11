use wasm_bindgen::prelude::*;
use getrandom::fill;

const SIZE:usize = 10;

#[wasm_bindgen]
struct State {
    minefield: [[ i32; SIZE]; SIZE]
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
     pub fn new() -> State {
        State{ minefield: [[0; SIZE]; SIZE]}
    }

    fn empty(&self) -> bool
    {
        for line in &self.minefield {
            for  c in  line {
                if *c > 0 {
                    return false;
                }
            }
        }
        true
    }

    fn bomb(&self, x: i32, y: i32) -> bool
    {
        self.minefield[x as usize][y as usize] > 0
    }

    fn setup(&mut self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        let mut buf: [u8; 30] = [0u8; 15*2];
        let _ = getrandom::fill(&mut buf);
        for i in 0usize..15 {
            let nx = buf[2*i]  as usize % SIZE;
            let ny = buf[2*i + 1]  as usize  % SIZE;
            if nx == x as usize && ny == y as usize{
                continue;
            }
            if self.minefield[nx][ny] == 0 {
                self.minefield[nx][ny] = 1;
                count += 1;
            }

        }
        count
    }

    #[wasm_bindgen]
    pub fn update(&mut self, x: i32, y:i32) -> i32
    {
        if self.empty() {
            self.setup(x,y);
        }
        if self.bomb(x,y) {
            return -1;
        } else {
            let mut k = 0;
            let ok = |x:i32| {return x >= 0 && x < SIZE as i32};

            for i in -1i32..2 {
                    for  j in -1i32..2 {
                        let xx:i32 = x+i;
                        let yy:i32 = y+j;
                        if ok(xx) && ok(yy) {
                            k += self.minefield[xx as usize][yy as usize];
                        }
                    }
                }
            // clicked(x,y,k);
            return k;
        }
    }

}


#[wasm_bindgen]
extern "C" {
    // pub fn clicked(x:i32, y:i32, text:&str);

}





