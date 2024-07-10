#include <memory>
#include "voro++.hh"

namespace voro
{
    template <typename T, typename... Args>
    inline std::unique_ptr<T> construct(Args... args)
    {
        return std::make_unique<T>(args...);
    }

    template <typename T>
    inline std::unique_ptr<T> clone_voronoicell(std::unique_ptr<T> const &c0)
    {
        auto c1 = construct<T>();
        // voronoicell only has a well-define copy assignment operator
        *c1 = *c0;
        return c1;
    }

    template <typename T>
    inline std::unique_ptr<T> clone_wall(std::unique_ptr<T> const &w0)
    {
        // wall is trivially copyable.
        return std::make_unique<T>(*w0);
    }

    inline wall &wall_sphere_to_wall(wall_sphere &w)
    {
        return w;
    }
    inline wall &wall_plane_to_wall(wall_plane &w)
    {
        return w;
    }
    inline wall &wall_cylinder_to_wall(wall_cylinder &w)
    {
        return w;
    }
    inline wall &wall_cone_to_wall(wall_cone &w)
    {
        return w;
    }
}