from jinja2 import Template
import glob
import re
import os

existing_days = glob.glob("./day*.py")
last_day = re.findall(r"(\d{2})", existing_days[-1])[0]
day = int(last_day) + 1

with open("day.jinja2") as f:
    template = Template(f.read())

with open(f"day{day:02d}.py", "w") as g:
    g.write(template.render(day=day))


inputs_dir = f"inputs/{day:02d}"
os.mkdir(inputs_dir)
open(f"{inputs_dir}/example_input.txt", "a").close()
open(f"{inputs_dir}/input.txt", "a").close()