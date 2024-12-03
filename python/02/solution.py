def solution():
    with open('input.txt', 'r') as file:
        lines = file.readlines()
    reports = [[int(s) for s in l.strip().split(' ')] for l in lines]
    print("part1: ", len([r for r in reports if is_safe(r)]))
    print("part2: ", len([r for r in reports if is_safe_2(r)]))


def is_safe(report):
    last_change = None
    for i in range(1, len(report)):
        change = report[i] - report[i-1]
        if change == 0 or abs(change) > 3:
            return False
        if last_change is not None and sign(change) != sign(last_change):
            return False
        last_change = change
    return True


def sign(n):
    return n > 0


def is_safe_2(report):
    return any(is_safe(report[:i] + report[i+1:]) for i in range(len(report)))


if __name__ == '__main__':
    solution()
