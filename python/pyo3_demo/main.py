#!/usr/bin/env python

import logging
from pyo3_demo import Tree

FORMAT = '%(levelname)s %(name)s %(asctime)s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)
tree = Tree()
tree.add_branch(0)
tree.add_branch(1, 0)
tree.add_branch(2, 0)
tree.add_leaf(0, 2)
tree.add_leaf(1, 3)
print(tree)
print(f"{tree.num_nodes} nodes in this tree")
