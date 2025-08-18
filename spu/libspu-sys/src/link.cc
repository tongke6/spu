#include "yacl/link/context.h"
#include "yacl/link/factory.h"
#include "libspu/device/io.h"
#include "libspu/core/pt_buffer_view.h"
#include "spu/libspu-sys/src/lib.rs.h"

namespace spu::device {
    std::unique_ptr<IoClient> new_io_client(size_t world_size, const spu::RuntimeConfig& config) {
        return std::make_unique<IoClient>(world_size, config);
    }

    rust::Vec<std::unique_ptr<spu::Value>> make_shares(const IoClient& io, const spu::PtBufferView& bv, spu::Visibility vtype, int32_t owner_rank) {
        auto shares = const_cast<IoClient&>(io).makeShares(bv, vtype, owner_rank);
        rust::Vec<std::unique_ptr<spu::Value>> rust_shares;
        for (auto& share : shares) {
            rust_shares.push_back(std::make_unique<spu::Value>(std::move(share)));
        }
        return rust_shares;
    }
}

namespace spu {
    std::unique_ptr<PtBufferView> new_pt_buffer_view(const uint8_t* ptr, PtType pt_type, const rust::Vec<int64_t>& shape, const rust::Vec<int64_t>& strides) {
        return std::make_unique<PtBufferView>(ptr, pt_type, spu::Shape(shape.begin(), shape.end()), spu::Strides(strides.begin(), strides.end()));
    }
}

namespace yacl::link {

std::unique_ptr<ContextDesc> new_context_desc() {
    return std::make_unique<ContextDesc>();
}

void add_party(std::unique_ptr<ContextDesc>& desc, rust::Str id, rust::Str host) {
    desc->parties.push_back({std::string(id), std::string(host)});
}

std::shared_ptr<Context> create_mem_link_context(const std::unique_ptr<ContextDesc>& desc, size_t self_rank) {
    auto lctx = FactoryMem().CreateContext(*desc, self_rank);
    lctx->ConnectToMesh();
    return lctx;
}

} // namespace yacl::link
