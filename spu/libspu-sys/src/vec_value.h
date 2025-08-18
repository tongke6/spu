#pragma once

#include <memory>
#include <vector>
#include "libspu/core/value.h"

namespace spu {
    using VecOfUniquePtrValue = std::vector<std::unique_ptr<spu::Value>>;
}
