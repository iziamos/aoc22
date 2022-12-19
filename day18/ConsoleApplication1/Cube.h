#pragma once

#include <string>
class Cube
{

	int x;
	int y;
	int z;
	int n;

	Cube(int x, int y, int z);


public:
	static Cube fromString(std::string s);
	std::string toString();
	bool isNeighbouring(Cube other);
	void recordNeighbour();
	int getOpenSides();
};
