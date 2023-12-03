import json
import re
import io

from dataclasses import dataclass, field, asdict
from typing import List, Tuple, Dict, DefaultDict, TextIO


@dataclass
class Isotope:
    mass: float
    abundance: float
    neutrons: int


@dataclass
class Element:
    symbol: str
    element_number: int
    most_abundant_isotope: int
    monoisotope: int
    isotopes: Dict[int, Isotope] = field(default_factory=dict)

    def to_dict(self) -> dict:
        state = asdict(self)
        # state["isotopes"] = {k: asdict(v) for k, v in state["isotopes"].items()}
        return state


@dataclass
class Entry:
    symbol: str
    element_number: int
    mass_number: int
    mass: float
    abundance: float


@dataclass
class NISTParser:
    elements: Dict[str, Element] = field(default_factory=dict)

    def parse(self, stream: TextIO):
        buffer = {}
        for line in stream:
            line = line.strip()
            if line.startswith("#"):
                continue
            if not line:
                if buffer:
                    self.process_buffer(buffer)
                    buffer.clear()
            else:
                k, v = line.split(" =")
                v = v.strip()
                buffer[k] = v

        if buffer:
            self.process_buffer(buffer)
            buffer.clear()

    def process_buffer(self, buffer: Dict[str, str]):
        entry = Entry(
            symbol=buffer["Atomic Symbol"],
            element_number=int(buffer["Atomic Number"]),
            mass_number=int(buffer["Mass Number"]),
            mass=float(buffer["Relative Atomic Mass"].split("(")[0]),
            abundance=float(buffer["Isotopic Composition"].split("(")[0] or 0.0),
        )
        if entry.symbol not in self.elements:
            monoisotope = None
            weights = buffer['Standard Atomic Weight'][1:-1].split(",")
            if len(weights) == 1:
                monoisotope = int(weights[0])
            element = Element(
                entry.symbol,
                element_number=entry.element_number,
                most_abundant_isotope=None,
                monoisotope=monoisotope,
            )
            self.elements[entry.symbol] = element
        else:
            element = self.elements[entry.symbol]
        iso = Isotope(
            entry.mass, entry.abundance, entry.mass_number
        )
        element.isotopes[iso.neutrons] = iso

    def index_isotopes(self):
        for sym, element in self.elements.items():
            if not element.monoisotope:
                most_abundant_isoform = max(element.values(), key=lambda x: x.abundance)


    def write_json(self, stream: TextIO):
        json.dump(
            {k: v.to_dict() for k, v in self.elements.items()},
            stream,
            sort_keys=True,
            indent=2,
        )


if __name__ == "__main__":
    import sys

    parser = NISTParser()
    parser.parse(sys.stdin)
    parser.write_json(sys.stdout)