#include <memory>

namespace voro
{
    template <typename T, typename... Args>
    inline std::unique_ptr<T> construct(Args... args)
    {
        return std::make_unique<T>(args...);
    }
}