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

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Copy + Default + Ord + std::ops::Rem<Output = T>,
{
    loop {
        if a == T::default() {
            return b;
        }
        if b == T::default() {
            return a;
        }
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy
        + Default
        + Ord
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(2, gcd(4, 2));
        assert_eq!(3, gcd(3, 3));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(6, lcm(2, 3));
        assert_eq!(12, lcm(4, 6));
    }
}
