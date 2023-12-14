// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Hash for Matrix<T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        // For now, we only store data in row-major order, so meh.
        self.data.hash(state);
    }
}

impl<T: Copy> Matrix<T> {
    pub fn new(width: usize, height: usize, default: T) -> Matrix<T> {
        Matrix {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        self.data[x + y * self.width] = v;
    }

    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.data.swap(x1 + y1 * self.width, x2 + y2 * self.width)
    }

    pub fn col(&self, x: usize) -> Col<T> {
        Col {
            matrix: self,
            x,
            y_low: 0,
            y_high: self.height,
        }
    }

    pub fn row(&self, y: usize) -> Row<T> {
        Row {
            matrix: self,
            x_low: 0,
            x_high: self.width,
            y,
        }
    }

    // TODO: Implement rotate and transposition.
}

pub struct Col<'a, T> {
    matrix: &'a Matrix<T>,
    x: usize,
    y_low: usize,
    y_high: usize,
}

impl<'a, T> Iterator for Col<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_low < self.y_high {
            let y = self.y_low;
            self.y_low += 1;
            Some(&self.matrix.data[self.x + y * self.matrix.width])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.y_high - self.y_low;
        (remaining, Some(remaining))
    }
}

impl<'a, T> DoubleEndedIterator for Col<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y_high > self.y_low {
            self.y_high -= 1;
            let y = self.y_high;
            Some(&self.matrix.data[self.x + y * self.matrix.width])
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Col<'a, T> {}

pub struct Row<'a, T> {
    matrix: &'a Matrix<T>,
    x_low: usize,
    x_high: usize,
    y: usize,
}

impl<'a, T> Iterator for Row<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x_low < self.x_high {
            let x = self.x_low;
            self.x_low += 1;
            Some(&self.matrix.data[x + self.y * self.matrix.width])
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.x_high - self.x_low;
        (remaining, Some(remaining))
    }
}

impl<'a, T> DoubleEndedIterator for Row<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x_high > self.x_low {
            self.x_high -= 1;
            let x = self.x_high;
            Some(&self.matrix.data[x + self.y * self.matrix.width])
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Row<'a, T> {}
