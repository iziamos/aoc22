#include "Cube.h"
#include <sstream>
#include <iostream>
#include <vector>
#include <memory>

using namespace std;

Cube::Cube(int x, int y, int z)
{
	this->x = x;
	this->y = y;
	this->z = z;
	this->n = 0;
}

Cube Cube::fromString(std::string s)
{
	int x, y, z;
	std::string buffer;
	std::istringstream iss(s);

	getline(iss, buffer, ',');
	x = stoi(buffer);
	getline(iss, buffer, ',');
	y = stoi(buffer);
	getline(iss, buffer, ',');
	z = stoi(buffer);

	return Cube(x, y, z);
}

std::string Cube::toString()
{
	// there is a god out there and he knows how the fuk people do this
	std::string ret("Cube {");
	ret.append("x = '");
	ret.append(std::to_string(x));
	ret.append("', ");

	ret.append("y = '");
	ret.append(std::to_string(y));
	ret.append("', ");

	ret.append("z = '");
	ret.append(std::to_string(z));
	ret.append("' }");

	return ret;
}

bool Cube::isNeighbouring(Cube other)
{
	return
		(x == other.x && y == other.y && (z == other.z - 1 || z == other.z + 1)) ||
		(x == other.x && z == other.z && (y == other.y - 1 || y == other.y + 1)) ||
		(y == other.y && z == other.z && (x == other.x - 1 || x == other.x + 1));
}

bool Cube::isSame(Cube other)
{
	return (this->x == other.x) &&
		(this->y == other.y) &&
		(this->z == other.z);
}

int Cube::getOpenSides()
{
	return 6 - n;
}

bool Cube::isAt(int x, int y, int z)
{
	return (this->x == x) && (this->y == y) && (this->z == z);
}

std::vector<shared_ptr<Cube>> Cube::getAdjacent()
{
	std::vector<shared_ptr<Cube>> ret;

	if (x > -1)  ret.push_back(std::make_shared<Cube>(Cube(x - 1, y, z)));
	if (x < 20) ret.push_back(std::make_shared<Cube>(Cube(x + 1, y, z)));
	if (y > -1)  ret.push_back(std::make_shared<Cube>(Cube(x, y - 1, z)));
	if (y < 20) ret.push_back(std::make_shared<Cube>(Cube(x, y + 1, z)));
	if (z > -1)  ret.push_back(std::make_shared<Cube>(Cube(x, y, z - 1)));
	if (z < 20) ret.push_back(std::make_shared<Cube>(Cube(x, y, z + 1)));

	return ret;
}

void Cube::recordNeighbour()
{
	n++;
}