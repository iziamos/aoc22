
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include "Cube.h"
#include <vector>


using namespace std;
int main(int argc, char ** argv)
{
    string line;
    ifstream file;

    file.open("input.txt");

    vector<Cube> cubes;
 
    while (getline(file, line)) 
    {
        auto c = Cube::fromString(line);
        cubes.push_back(c);
    }

    for (int i = 0; i < cubes.size() - 1; ++i)
    {
        for (int j = i + 1; j < cubes.size(); ++j)
        {
            if (cubes[i].isNeighbouring(cubes[j]))
            {
                cubes[i].recordNeighbour();
                cubes[j].recordNeighbour();
            }
        }
    }

    int t = 0;
    for (Cube& c : cubes)
    {
        t += c.getOpenSides();
    }

    cout << "The number of open sides is: " << t << endl;
    return 0;
}
