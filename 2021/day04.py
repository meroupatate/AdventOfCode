from helpers.utils import * 

DAY = 4

def parse_board(lines):
    board = []
    for line in lines:
        board.append([[int(l.strip()), False] for l in line.split()])
    return board

def display_board(board_lines):
    print("")
    for line in board_lines:
        for cell in line:
            value, found = cell
            if found:
                print(f"\033[92m{value:2d} \033[0m", end="")
            else:
                print(f"{value:2d} ", end="")
        print("")
            

def display_boards(boards):
    for board in boards:
        display_board(board)
        

def find_value(boards, value):
    for board in boards:
        for lines in board:
            for cell in lines:
                if cell[0] == value:
                    cell[1] = True


def check_winner(boards):
    for board in boards:
        if won(board):
            return board
    return None


def get_winners(boards):
    winners = []
    for board in boards:
        if won(board):
            winners.append(board)
    return winners



def won(board):
    for line in board:
        if all([cell[1] for cell in line]):
            return True
    for c in range(len(board[0])):
        column = [line[c] for line in board]
        if all([cell[1] for cell in column]):
            return True
    return False


def get_unmarked_numbers_sum(board):
    res = 0
    for line in board:
        for cell in line:
            if not cell[1]:
                res += cell[0]
    return res


def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    drawing = list(map(int, lines[0].split(",")))
    boards = []
    for l in range(1, len(lines), 6):
        boards.append(parse_board(lines[l+1:l+6]))

    for draw in drawing:
        find_value(boards, draw)
        #display_boards(boards)
        winner = check_winner(boards)
        if winner:
            display_board(winner)
            return draw * get_unmarked_numbers_sum(winner)


print("== FIRST PART ==")
assert_example(first_part, 4512)
get_result(first_part)


def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    drawing = list(map(int, lines[0].split(",")))
    boards = []
    for l in range(1, len(lines), 6):
        boards.append(parse_board(lines[l+1:l+6]))
    
    for draw in drawing:
        find_value(boards, draw)
        winners = get_winners(boards)
        for winner in winners:
            boards.remove(winner)
            if len(boards) == 0:
                display_board(winner)
                return draw * get_unmarked_numbers_sum(winner)
        


print("== SECOND PART ==")
assert_example(second_part, 1924)
get_result(second_part)