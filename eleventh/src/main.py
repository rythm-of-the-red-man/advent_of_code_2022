
class Operations:
    ADD = "ADD"
    MULTIPLY = "MULTIPLY"
    SQUARE = "SQUARE"
import time

# def square(x, y=2):
#     z = 0
#     while y > 0:
#         if y % 2 == 1:
#             z = z + x
#         x = x << 1
#         y = y >> 1
#     return z


class Monkey:
    def __init__(
        self,
        index=0,
        inspected_items=0,
        current_items=[],
        test_number=0,
        true_monkey=0,
        false_monkey=0,
        operation_number=0,
        operation_type=Operations.ADD,
    ) -> None:
        self.index = index
        self.inspected_items = inspected_items
        self.current_items = current_items
        self.test_number = test_number
        self.true_monkey = true_monkey
        self.false_monkey = false_monkey
        self.operation_number = operation_number
        self.operation_type = operation_type

    def get_item(self, item):
        self.current_items.append(item)

    def perform_operation(self, old):
        if self.operation_type == Operations.ADD:
            return old + self.operation_number
        elif self.operation_type == Operations.MULTIPLY:
            return old * self.operation_number
        elif self.operation_type == Operations.SQUARE:
            # if old > 2147483647:
            #     breakpoint()
            return old ** 2

    def perform_test(self, test_val,factor):
        # result = test_val //3
        result = test_val % factor
        print(test_val, result)
        return (result, result % self.test_number == 0)

    def round(self, monkeys,factor):
        for item in self.current_items:
            new = self.perform_operation(item)
            new_test = self.perform_test(new,factor)
            self.inspected_items += 1
            if new_test[1]:
                monkeys[self.true_monkey].get_item(new_test[0])
            else:
                monkeys[self.false_monkey].get_item(new_test[0])
        self.current_items = []


def load_monkeys(monkey_list):
    index = 0
    with open("./input.txt", 'r') as f:
        lines = f.readlines()
        for line in lines:
            if "  Starting items: " in line:
                items = line.replace("  Starting items: ", '')
                monkey_list.append(
                    Monkey(
                        index, current_items=[int(item) for item in items.split(',')]
                    )
                )
                index += 1

            elif "  Test: divisible by " in line:
                number = line.replace("  Test: divisible by ", '')
                monkey_list[index - 1].test_number = int(number)
            elif "  Operation: new = old + " in line:
                number = line.replace("  Operation: new = old + ", '')
                monkey_list[index - 1].operation_type = Operations.ADD
                monkey_list[index - 1].operation_number = int(number)
            elif "    If false: throw to monkey " in line:
                number = line.replace("    If false: throw to monkey ", '')
                monkey_list[index - 1].false_monkey = int(number)
            elif "    If true: throw to monkey " in line:
                true_monkey = line.replace("    If true: throw to monkey ", '')
                monkey_list[index - 1].true_monkey = int(true_monkey)
            elif "  Operation: new = old * old" in line:
                monkey_list[index - 1].operation_type = Operations.SQUARE
                monkey_list[index - 1].operation_number = -1
            elif "  Operation: new = old * " in line:
                number = line.replace("  Operation: new = old * ", '')
                monkey_list[index - 1].operation_type = Operations.MULTIPLY
                monkey_list[index - 1].operation_number = int(number)


if __name__ == "__main__":
    monkey_list = []
    load_monkeys(monkey_list)
    factor = 1
    for monkey in monkey_list:
         factor *= monkey.test_number
    for i in range(0, 10000):
        print(i)
        if i in [1, 20, 1000, 2000,3000,4000,5000]:
            print([monkey.inspected_items for monkey in monkey_list])
        for monkey in monkey_list:
            monkey.round(monkey_list,factor)

    foo = [monkey.inspected_items for monkey in monkey_list]
    foo.sort()
    # print(foo)
    print(foo[-1] * foo[-2])

# 14050198470
# 14636993466
# 2713310158
