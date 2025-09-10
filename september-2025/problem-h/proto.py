# /// script
# requires-python = ">=3.11"
# dependencies = [
# ]
# ///
# pyright: strict
from dataclasses import dataclass
from pathlib import Path

HOME = "BATUMI"


@dataclass(slots=True, kw_only=True)
class State:
    used_tickets: int  # max 100 tickets
    position: int
    bought: list[tuple[int, int]]

    def push(self, ticket: int) -> None:
        self.used_tickets |= 1 << ticket

    def pop(self, ticket: int) -> None:
        self.used_tickets &= ~(1 << ticket)

    def has(self, ticket: int) -> bool:
        return self.used_tickets & (1 << ticket) != 0

    def count(self) -> int:
        return self.used_tickets.bit_count()


class Searcher:
    def __init__(self, tickets: list[list[str]]):
        self.seen: dict[tuple[int, int], int] = {}

        self.best_bought_count = 1 << 64  # large num, basically infinity
        self.best_bought = []

        nodes = {HOME}
        for ticket in tickets:
            for node in ticket:
                nodes.add(node)
        self.nodes = sorted(nodes)
        self.node_map = {n: i for i, n in enumerate(self.nodes)}

        self.home = self.node_map[HOME]
        self.tickets = [
            min((self.node_map[a], self.node_map[b]), (self.node_map[b], self.node_map[a]))
            for a, b in tickets
        ]
        self.state = State(used_tickets=0, position=self.home, bought=[])

        self.candidate_cache = {
            node: sorted(enumerate(self.tickets), key=lambda item: node in item[1], reverse=True)
            for node in range(len(self.nodes))
        }

    def search(self) -> None:
        if self.state.count() == len(self.tickets):
            must_return = self.state.position != self.home
            bought_count = len(self.state.bought) + must_return
            if bought_count < self.best_bought_count:
                print(f"\tNew best: {bought_count}")
                self.best_bought = self.state.bought.copy()
                if must_return:
                    self.best_bought.append((self.state.position, self.home))
                self.best_bought_count = bought_count

            return

        key = (self.state.count(), self.state.position)
        if self.seen.get(key, 1 << 64) < len(self.state.bought):
            return
        self.seen[key] = len(self.state.bought)

        if len(self.state.bought) > self.best_bought_count:
            return

        used_tickets: set[tuple[int, int]] = set()
        for i, ticket in self.candidate_cache[self.state.position]:
            if self.state.has(i):
                continue
            if ticket in used_tickets:
                # dedup
                continue
            used_tickets.add(ticket)

            # Try to use ticket
            self.state.push(i)
            prev_position = self.state.position

            if self.state.position not in ticket:
                for destination in ticket:
                    self.state.bought.append((self.state.position, destination))
                    self.state.position = destination
                    self.search()
                    self.state.bought.pop()
            else:
                self.state.position = ticket[0] if ticket[0] != self.state.position else ticket[1]
                self.search()

            self.state.position = prev_position
            self.state.pop(i)


def main() -> None:
    input_path = Path(__file__).parent / "src" / "input.txt"
    output_path = Path(__file__).parent / "output.txt"
    data = iter(input_path.read_text().splitlines())
    cases = int(next(data))
    with output_path.open("w", newline="\n", encoding="utf-8") as f:
        for i in range(cases):
            dot_path = Path(__file__).parent / f"case_{i}.dot"
            ticket_count = int(next(data))
            tickets = [next(data).split(" ", 1) for _ in range(ticket_count)]
            print("Start:", tickets)

            with dot_path.open("w", newline="\n", encoding="utf-8") as f2:
                print("graph {", file=f2)
                for src, dst in tickets:
                    print(f"  {src} -- {dst}", file=f2)
                print("}", file=f2)

            searcher = Searcher(tickets)
            searcher.search()
            print(f"Case #{i + 1}: ", searcher.best_bought_count, file=f)
            for src, dst in searcher.best_bought:
                print(searcher.nodes[src], searcher.nodes[dst], file=f)

            print(f"Case #{i + 1}: ", searcher.best_bought_count)
            for src, dst in searcher.best_bought:
                print(searcher.nodes[src], searcher.nodes[dst])


if __name__ == "__main__":
    main()
