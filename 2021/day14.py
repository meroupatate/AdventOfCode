from helpers.utils import * 
import re

DAY = 14

class Polymer:
    def __init__(self, lines):
        self.template = list(lines[0])
        rules = [
            re.findall('^([A-Z]{2}) -> ([A-Z])' ,line)[0] for line in lines[2:]
        ] 
        self.rules = {
            pair: result for pair, result in rules 
        }
        self.steps = 0

    def step(self):
        next_template = self.template.copy()
        inserted = 0
        for i in range(len(self.template)-1):
            pair = ''.join(self.template[i:i+2])
            if pair in self.rules.keys():
                next_template = next_template[:i+inserted+1] + [self.rules[pair]] + next_template[i+inserted+1:]
                inserted += 1
        self.template = next_template
        self.steps += 1
    
    def count_elements(self):
        counter = {}
        for char in self.template:
            if char not in counter.keys():
                counter[char] = 0
            counter[char] += 1
        return counter
    
    def get_most_common_element_count(self):
        counter = self.count_elements()
        return max(counter.values())
    
    def get_least_common_element_count(self):
        counter = self.count_elements()
        return min(counter.values())
            
    

def first_part(input_file):
    lines = extract_lines(DAY, input_file)
    polymer = Polymer(lines)
    for _ in range(10):
        polymer.step()
    return polymer.get_most_common_element_count() - polymer.get_least_common_element_count()
    

print("== FIRST PART ==")
assert_example(first_part, 1588)
get_result(first_part)


class PolymerTryAgain:
    def __init__(self, lines):
        self.rules = {
            pair: result for pair, result in self.parse_lines(lines)
        }
        self.template = list(lines[0])
        self.count_elements = {}
        self.count_pairs = {}
        self.populate_counts()

        self.steps = 0

    def parse_lines(self, lines):
        return [
            re.findall('^([A-Z]{2}) -> ([A-Z])' ,line)[0] for line in lines[2:]
        ]

    def populate_counts(self):
        for i, elem in enumerate(self.template):
            if elem not in self.count_elements:
                self.count_elements[elem] = 0
            self.count_elements[elem] += 1

            if i != len(self.template) - 1:
                pair = "".join(self.template[i:i+2])
                if pair not in self.count_pairs:
                    self.count_pairs[pair] = 0
                self.count_pairs[pair] += 1

    def step(self):
        count_pairs = {}
        for pair in self.count_pairs.keys():
            new_elem = self.rules[pair]
            times = self.count_pairs[pair]
            self.increment_element(new_elem, times)

            new_pairs = [pair[0]+new_elem, new_elem+pair[1]]
            for pair in new_pairs:
                if pair not in count_pairs:
                    count_pairs[pair] = 0
                count_pairs[pair] += times
        self.count_pairs = count_pairs

        self.steps += 1
    
    def increment_element(self, element, times):
        if element not in self.count_elements:
            self.count_elements[element] = 0
        self.count_elements[element] += times

    def get_most_common_element_count(self):
        return max(self.count_elements.values())
    
    def get_least_common_element_count(self):
        return min(self.count_elements.values())

def second_part(input_file):
    lines = extract_lines(DAY, input_file)
    polymer = PolymerTryAgain(lines)
    for _ in range(40):
        polymer.step()
    return polymer.get_most_common_element_count() - polymer.get_least_common_element_count()


print("== SECOND PART ==")
assert_example(second_part, 2188189693529)
get_result(second_part)