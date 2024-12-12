# Minmax Queue

В этой задаче мы предлагаем вам написать очередь с операцией "взять минимум/максимум".

## Реализация

Очередь должна поддерживать константную асимптотику как на добавление/удаление элементов, так и на взятие максимума и минимума.

Интерфейс должен быть следующий (шаблон уже находится в папке src/lib.rs):

* `fn new()` -- создание пустой очереди
* `fn push(x: i32)` -- добавить число в очередь
* `fn pop() -> i32` -- достать число из очереди
* `fn min() -> Option<i32>` -- найти минимум
* `fn max() -> Option<i32>` -- найти максимум
* `fn first() -> Option<i32>` -- вернуть первый элемент
* `fn last() -> Option<i32>` -- вернуть последний элемент
* `fn len() -> usize` -- вернуть количество элементов в очереди
* `fn is_empty() -> bool` -- пустая ли очередь