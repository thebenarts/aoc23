#include <sstream>
#include <fstream>
#include <iostream>
#include <string_view>
#include <vector>

int ToNumber(std::string_view view);
bool IsNumber(const char character);
int ViewToString(std::string_view view);

int ViewToString(std::string_view view)
{
    if (view.empty())
    {
        return -1;
    }
    
    if (const auto num{ ToNumber(view.substr(0,1)) }; num != -1)
    {
        return num;
    }

    if (view.size() >= 3)
    {
        const auto threeView{ view.substr(0,3) };
        if ("one" ==threeView)
        {
            return 1;
        }
        else if ("two" ==threeView)
        {
            return 2;
        }
        else if ("six" == threeView)
        {
            return 6;
        }
    }

    if (view.size() >= 4)
    {
        const auto fourView{ view.substr(0,4) };
        if ("four" == fourView)
        {
            return 4;
        }
        else if ("five" == fourView)
        {
            return 5;
        }
        else if ("nine" == fourView)
        {
            return 9;
        }
    }

    if (view.size() >= 5)
    {
        const auto fiveView{ view.substr(0,5) };
        if ("three" == fiveView)
        {
            return 3;
        }
        else if ("seven" == fiveView)
        {
            return 7;
        }
        else if ("eight" == fiveView)
        {
            return 8;
        }
    }

    return -1;
}

int ToNumber(std::string_view view)
{
    if (view.empty())
    {
        return -1;
    }

    int number{};
    for (const char c : view)
    {
        if (!IsNumber(c))
        {
            return -1;
        }
        number *= 10;
        number += c & 0x0F;
    }

    return number;
}

bool IsNumber(const char character)
{
    return character >= '0' && character <= '9';
}

int main()
{
    std::fstream input{ "E:\\dev\\aoc23\\input\\d1.txt" };
    std::string line;
    std::vector<int> numbers;
    int nums[2];
    int iterNum{ 0 };
    while(std::getline(input, line))
    {
        int num{};
        std::string_view view{ line };
        for (int i = 0; i < line.size(); i++)
        {
            view = { line.data() + i, line.size() - i};
            num = ViewToString(view);
            if (num != -1)
            {
                break;
            }
        }

        nums[0] = num;

        for (int i = line.size() - 1; i >= 0; i--)
        {
            view = { line.data() + i, line.size() - i };
            num = ViewToString(view);
            if (num != -1)
            {
                break;
            }
        }

        nums[1] = num;
        numbers.push_back(nums[0] * 10 + nums[1]);
        iterNum++;
    }

    int result{};
    for (auto num : numbers)
    {
        result += num;
    }
    std::cout << result;
}
