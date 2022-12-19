#pragma once

#include <string>
#include <vector>
#include <memory>


class Cube
{

	int x;
	int y;
	int z;
	int n;




public:
	Cube(int x, int y, int z);
	static Cube fromString(std::string);
	std::string toString();
	bool isNeighbouring(Cube);
	bool isSame(Cube);
	void recordNeighbour();
	int getOpenSides();
	bool isAt(int, int, int);
	std::vector<std::shared_ptr<Cube>>  getAdjacent();
};
