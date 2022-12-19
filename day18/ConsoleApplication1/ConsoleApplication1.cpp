
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include "Cube.h"
#include <vector>
#include<memory>


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
    int z = 0;
    vector<Cube> not_enclosed;

    for (Cube& c : cubes)
    {
        t += c.getOpenSides();
        if (c.getOpenSides() > 0) {
            not_enclosed.push_back(c);
        }
    }


    // loss of patience so crack out the hackery

    vector<shared_ptr<Cube>> air;
    auto root = std::make_shared<Cube>(Cube::fromString("1,1,1"));
    air.push_back(root);

    for (;;)
    {
        auto s = air.size();

        vector<shared_ptr<Cube>> temp;
        for (auto &c : air)
        {
            auto cube = c.get();
            vector<shared_ptr<Cube>> neighbours = cube->getAdjacent();
            for (auto n : neighbours)
            {
                auto neighbour = n.get();
                bool already_in_air = false;
                for (auto &o : air)
                {
                    auto other = o.get();
                    if (other->isSame(*neighbour))
                    {
                        already_in_air = true;
                        break;
                    }
                }

                for (auto& o : temp)
                {
                    auto other = o.get();
                    if (other->isSame(*neighbour))
                    {
                        already_in_air = true;
                        break;
                    }
                }

                for (auto& c : not_enclosed)
                {
                    if (c.isSame(*neighbour))
                    {
                        already_in_air = true;
                        break;
                    }
                }

                if (!already_in_air)
                {
                    temp.push_back(n);
                }
            }
        }
        for (auto n : temp)
        {
            air.push_back(n);
        }

        if (air.size() == s)
        {
            break;
        }
    }

    int result = 0;
    for (auto ne : not_enclosed)
    {
        auto neighbours = ne.getAdjacent();
        for (auto neighba : neighbours)
        {
            for (auto airman : air)
            {
                if (neighba.get()->isSame(*(airman.get())))
                {
                    ++result;
                    break;
                }
            }
        }
    }


    cout << "The number of open sides is: " << result << endl;
    return 0;
}
