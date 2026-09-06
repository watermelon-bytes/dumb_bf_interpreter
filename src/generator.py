import sys
# Ignoring case where pointer goes out-of-bounds
# (only considerable for very long strings, 10'000+ characters)
def generate_bf_code(s: str):
    output_string = ""
    for char in s:
        output_string += "+" * ord(char)
        output_string += ".>\n"
    print(output_string)


if __name__ == "__main__":
    input_string = ""
    if len(sys.argv) == 1:
        print("At least one string is required")
        sys.exit(1)

    for i in range(1, len(sys.argv)):
        input_string += sys.argv[i] + " "
    generate_bf_code(input_string)
    sys.exit(0)
