/*
 * Copyright 2022 Google Inc. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub struct Generator {
    width: u32,
    height: u32,
    palette: Box<[[u8; 4]]>,
}

impl Generator {
    pub fn new(width: u32, height: u32, max_iterations: u32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            width,
            height,
            palette: (0..max_iterations)
                .map(move |_| {
                    let (r, g, b) = hsl::HSL {
                        h: rand::Rng::gen_range(&mut rng, 0.0..360.0),
                        s: 0.5,
                        l: 0.6,
                    }
                    .to_rgb();
                    [r, g, b, 255]
                })
                .collect(),
        }
    }

    fn get_color(&self, x: u32, y: u32) -> &[u8; 4] {
        let c = num_complex::Complex64::new(
            (f64::from(x) - f64::from(self.width) / 2.0) * 4.0 / f64::from(self.width),
            (f64::from(y) - f64::from(self.height) / 2.0) * 4.0 / f64::from(self.height),
        );
        let mut z = num_complex::Complex64::new(0.0, 0.0);
        let mut i = 0;
        while z.norm_sqr() < 4.0 {
            if i == self.palette.len() {
                return &self.palette[0];
            }
            z = z.powi(2) + c;
            i += 1;
        }
        &self.palette[i]
    }

    fn iter_row_bytes(&self, y: u32) -> impl '_ + Iterator<Item = u8> {
        (0..self.width)
            .flat_map(move |x| self.get_color(x, y))
            .copied()
    }

    pub fn iter_bytes(&self) -> impl '_ + Iterator<Item = u8> {
        (0..self.height).flat_map(move |y| self.iter_row_bytes(y))
    }
}