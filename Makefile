CWD = $(shell pwd)


dev:
	maturin develop
	python $(CWD)/python/pyo3_demo/main.py

test:
	python $(CWD)/python/pyo3_demo/main.py
