//
// Created by zhongyang on 3/11/23.
//
#include <variant>
#include <vector>
#include <functional>
#include <variant>
#include <iostream>

template<typename T>
class comparator_builder {
public:
    using Field = std::variant<int T::*, double T::*, std::string (T::*)() const, double (T::*)() const>;

    // Define a member function for specifying a field or method to compare
    comparator_builder<T> &by(Field f) {
        fields_.emplace_back(f);
        return *this;
    }

    // Define a member function for generating the comparator function
    std::function<bool(const T &, const T &)> operator()() {
        return [fields = fields_](const T &a1, const T &a2) -> bool {
            for (const auto &f: fields) {
                struct equaller {
                    const T &a;
                    const T &b;

                    bool operator()(int T::* field) const {
                        return a.*field == b.*field;
                    }

                    bool operator()(double T::* field) const {
                        return a.*field == b.*field;
                    }

                    bool operator()(std::string (T::*method)() const) const {
                        return (a.*method)() == (b.*method)();
                    }

                    bool operator()(double (T::*method)() const) const {
                        return (a.*method)() == (b.*method)();
                    }
                } e{a1, a2};
                if (std::visit(e, f)) {
                    continue;
                } else {
                    struct larger {
                        const T &a;
                        const T &b;

                        bool operator()(int T::* field) const {
                            return a.*field > b.*field;
                        }

                        bool operator()(double T::* field) const {
                            return a.*field > b.*field;
                        }

                        bool operator()(std::string (T::*method)() const) const {
                            return (a.*method)() > (b.*method)();
                        }

                        bool operator()(double (T::*method)() const) const {
                            return (a.*method)() > (b.*method)();
                        }
                    } l{a1, a2};
                    return std::visit(l, f);
                }
            }
            return false;
        };
    }

private:
    std::vector<Field> fields_;
};

struct MyStruct {
public:
    int x1;

    MyStruct(int x1, double x2, const std::string &x3) : x1(x1), x2(x2), x3(x3) {}

    double get_x2() const { return x2; }

    std::string get_x3() const { return x3; }

private:
    double x2;
    std::string x3;
};

int main() {
    std::vector<MyStruct> v{
            MyStruct{1, 2.0, "hello"},
            MyStruct{2, 1.0, "world"},
            MyStruct{1, 3.0, "hello"},
            MyStruct{1, 2.0, "world"},
    };

    std::sort(v.begin(), v.end(), comparator_builder<MyStruct>()
            .by(&MyStruct::x1)
            .by(&MyStruct::get_x2)());

    for (MyStruct ms: v) {
        std::cout << ms.x1 << " " << ms.get_x2() << " " << ms.get_x3() << std::endl;
    }
}