#include "libspu/spu.h"
#include "libspu/core/context.h"
#include "spu/libspu-sys/src/lib.rs.h"

namespace spu {
std::unique_ptr<RuntimeConfig> new_runtime_config(ProtocolKind protocol,
                                                  FieldType field,
                                                  int64_t fxp_fraction_bits) {
  return std::make_unique<RuntimeConfig>(protocol, field, fxp_fraction_bits);
}

std::unique_ptr<SPUContext> new_spu_context(const RuntimeConfig& config, std::shared_ptr<yacl::link::Context> lctx) {
    return std::make_unique<SPUContext>(config, lctx);
}
} // namespace spu
