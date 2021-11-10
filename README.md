# Birthday

[![build badge](https://github.com/LJason77/Birthday/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/LJason77/Birthday/actions/workflows/rust.yml)
![GitHub forks](https://img.shields.io/github/forks/LJason77/Birthday?style=social)
![GitHub Repo stars](https://img.shields.io/github/stars/LJason77/Birthday?style=social)

现假定一年只有 365 天，不考虑闰年情况。

按下列要求编程模拟检验: 

1. 编写函数 `create_birthday()`：使用随机函数生成某人的生日，结果为字符串，由 `01.01` 到 `12.31` 表示，月份.天均为两位数，中间用点号 `.` 隔开；
2. 编写函数 `group_birthdays(n)`：调用 `create_birthday()` 生成团队成员的生日，用列表方式返回，其中：n 为团队成员数；
3. 主函数：输入团队成员数，调用 `group_birthdays(n)`，模拟 100000 次，计算至少有两人生日相同的比例。

## 许可

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![GitHub](https://img.shields.io/github/license/LJason77/Birthday)
