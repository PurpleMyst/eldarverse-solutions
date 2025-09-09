from pathlib import Path

def main() -> None:
    base = Path(__file__).parent
    sample_input = base / "src" / "sample_input.txt"
    t = ["N/A"]
    t.extend(str(i) for i in range(1, 1 << 12))
    sample_input.write_text("\n".join(t))


if __name__ == "__main__":
    main()
