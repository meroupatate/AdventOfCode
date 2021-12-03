with open("input.txt", "r") as f:
    lines = [line.strip() for line in f.readlines()]
    seat_ids = set()
    for line in lines:
        # find seat row
        start = 0
        end = 127
        for i in range(7):
            if line[i] == "F":
                end = (end + start) // 2
            elif line[i] == "B":
                start = (end + start) // 2 + 1
        assert start == end
        row = start

        # find seat column
        start = 0
        end = 7
        for i in range(7, 10):
            if line[i] == "L":
                end = (end + start) // 2
            elif line[i] == "R":
                start = (end + start) // 2 + 1
        assert start == end
        column = start

        seat_id = row * 8 + column
        seat_ids.add(seat_id)
    for sid in seat_ids:
        if sid + 1 not in seat_ids and sid + 2 in seat_ids:
            print(f"My seat is #{sid + 1}")