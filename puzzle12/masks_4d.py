from functools import reduce
from operator import or_


def main() -> None:
    masks = set()

    def bitmask(p):
        x, y, z, w = p
        return 1 << ((x * 5 * 5 + y * 5 + z) * 5 + w)

    def tostr(m):
        return f"[{','.join(str(i) for i, c in enumerate(reversed(bin(m)[2:])) if c == '1')}]"

    for x in range(0, 5):
        for y in range(0, 5):
            for z in range(0, 5):
                for w in range(0, 5):
                    dirs = [
                        (dx, dy, dz, dw)
                        for dx in range(-1, 2)
                        for dy in range(-1, 2)
                        for dz in range(-1, 2)
                        for dw in range(-1, 2)
                        if (dx != 0 or dy != 0 or dz != 0)
                    ]

                    for dx, dy, dz, dw in dirs:
                        cells = [
                            (x + dx * i, y + dy * i, z + dz * i, w + dw * i)
                            for i in range(0, 5)
                        ]
                        if not all(0 <= coord < 5 for cell in cells for coord in cell):
                            continue
                        masks.add(reduce(or_, map(bitmask, cells)))

    print(f"[{','.join(map(tostr, sorted(masks)))}]")


if __name__ == "__main__":
    main()
