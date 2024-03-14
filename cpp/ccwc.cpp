#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

using namespace std;

int getWordCount(std::ifstream& inputFile)
{
    string line;
    int wordCount = 0;
    while (getline(inputFile, line)) {
        std::istringstream iss(line);
        std::string word;
        while (iss >> word) {
            wordCount++;
        }
    }
    return wordCount;
}

int getLineCount(std::ifstream& inputFile)
{
    string line;
    int lineCount = 0;

    while (getline(inputFile, line)) {
        ++lineCount;
    }
    return lineCount;
}

pair<int, int> getWordAndLineCount(std::ifstream& inputFile)
{
    string line;
    int lineCount = 0;
    int wordCount = 0;

    while (getline(inputFile, line)) {
        ++lineCount;
        std::istringstream iss(line);
        std::string word;
        while (iss >> word) {
            wordCount++;
        }
    }
    return { wordCount, lineCount };
}

void printAll(std::ifstream& inputFile, const string& filePath)
{
    inputFile.seekg(0, ios::end);
    streampos fileSize = inputFile.tellg();
    inputFile.seekg(0, ios::beg);

    auto [wordCount, lineCount] = getWordAndLineCount(inputFile);
    cout << lineCount << " " << wordCount << " " << fileSize << " " << filePath << endl;
}

void printBytes(std::ifstream& inputFile, const string& filePath)
{
    inputFile.seekg(0, ios::end);
    streampos fileSize = inputFile.tellg();
    inputFile.seekg(0, ios::beg);

    cout << fileSize << " " << filePath << endl;
}

void printLines(std::ifstream& inputFile, const string& filePath)
{
    int lineCount = getLineCount(inputFile);
    cout << lineCount << " " << filePath << endl;
}

void printWords(std::ifstream& inputFile, const string& filePath)
{
    int wordCount = getWordCount(inputFile);
    cout << wordCount << " " << filePath << endl;
}

int processFile(const string& option, const string& filePath)
{
    ifstream inputFile(filePath);

    if (!inputFile.is_open()) {
        cerr << "Error opening the file!" << endl;
        return 1;
    }

    try {
        if (option == "-c" || option == "--bytes") {
            printBytes(inputFile, filePath);
        } else if (option == "-l" || option == "--lines") {
            printLines(inputFile, filePath);
        } else if (option == "-a") {
            printAll(inputFile, filePath);
        } else if (option == "-w" || option == "--words") {
            printWords(inputFile, filePath);
        } else {
            cerr << "Unkown flag " << option << endl;
        }

        return 0;
    } catch (const std::ifstream::failure& e) {
        cerr << "Error reading file " << e.what() << endl;
        return 1;
    }
}

int main(int argc, char* argv[])
{
    if (argc < 2 || argc > 3) {
        cerr << "Usage: " << argv[0] << " [OPTION] <filePath>\n"
             << "\n"
             << "OPTIONS:\n"
             << "   -c, --bytes\n"
             << "       print the byte counts\n"
             << "   -l, --lines\n"
             << "       print the newline counts\n"
             << "   -w, --words\n"
             << "       print the word counts\n"
             << endl;
        return 1;
    }

    string option = (argc == 3) ? argv[1] : "-a";
    string filePath = argv[argc - 1];
    processFile(option, filePath);

    return 0;
}
