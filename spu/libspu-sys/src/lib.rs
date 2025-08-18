#[cxx::bridge(namespace = "spu")]
pub mod ffi {
    // The SPU datatype
    enum DataType {
      DT_INVALID,
      DT_I1,
      DT_I8,
      DT_U8,
      DT_I16,
      DT_U16,
      DT_I32,
      DT_U32,
      DT_I64,
      DT_U64,
      DT_F16,
      DT_F32,
      DT_F64,
    }

    // The visibility type.
    enum Visibility {
      VIS_INVALID,
      VIS_SECRET,
      VIS_PUBLIC,
      VIS_PRIVATE,
    }

    // Plaintext type
    enum PtType {
      PT_INVALID,
      PT_I8,
      PT_U8,
      PT_I16,
      PT_U16,
      PT_I32,
      PT_U32,
      PT_I64,
      PT_U64,
      PT_I128,
      PT_U128,
      PT_I1,
      PT_F16,
      PT_F32,
      PT_F64,
      PT_CF32,
      PT_CF64,
    }

    // A security parameter type.
    enum FieldType {
      FT_INVALID,
      FM32,
      FM64,
      FM128,
    }

    // The protocol kind.
    enum ProtocolKind {
      PROT_INVALID,
      REF2K,
      SEMI2K,
      ABY3,
      CHEETAH,
      SECURENN,
      SWIFT,
    }

    struct ClientSSLConfig {
      certificate: String,
      private_key: String,
      ca_file_path: String,
      verify_depth: i32,
    }

    struct TTPBeaverConfig {
      server_host: String,
      adjust_rank: i32,
      asym_crypto_schema: String,
      server_public_key: String,
      transport_protocol: String,
      ssl_config: SharedPtr<ClientSSLConfig>,
    }

    enum CheetahOtKind {
        YACL_Ferret,
        YACL_Softspoken,
        EMP_Ferret,
    }

    struct CheetahConfig {
      disable_matmul_pack: bool,
      enable_mul_lsb_error: bool,
      ot_kind: CheetahOtKind,
    }

    // The SPU runtime configuration.
    struct RuntimeConfig {
        protocol: ProtocolKind,
        field: FieldType,
        fxp_fraction_bits: i64,
        max_concurrency: i32,
        enable_action_trace: bool,
        enable_type_checker: bool,
        enable_pphlo_trace: bool,
        enable_runtime_snapshot: bool,
        snapshot_dump_dir: String,
        enable_pphlo_profile: bool,
        enable_hal_profile: bool,
        public_random_seed: u64,
        share_max_chunk_size: u64,
        sort_method: SortMethod,
        quick_sort_threshold: i64,
        fxp_div_goldschmidt_iters: i64,
        fxp_exp_mode: ExpMode,
        fxp_exp_iters: i64,
        fxp_log_mode: LogMode,
        fxp_log_iters: i64,
        fxp_log_orders: i64,
        sigmoid_mode: SigmoidMode,
        enable_lower_accuracy_rsqrt: bool,
        sine_cosine_iters: i64,
        beaver_type: BeaverType,
        ttp_beaver_config: SharedPtr<TTPBeaverConfig>,
        cheetah_2pc_config: CheetahConfig,
        trunc_allow_msb_error: bool,
        experimental_disable_mmul_split: bool,
        experimental_enable_inter_op_par: bool,
        experimental_enable_intra_op_par: bool,
        experimental_disable_vectorization: bool,
        experimental_inter_op_concurrency: u64,
        experimental_enable_colocated_optimization: bool,
        experimental_enable_exp_prime: bool,
        experimental_exp_prime_offset: u32,
        experimental_exp_prime_disable_lower_bound: bool,
        experimental_exp_prime_enable_upper_bound: bool,
    }

    enum SortMethod {
        SORT_DEFAULT,
        SORT_RADIX,
        SORT_QUICK,
        SORT_NETWORK,
    }

    enum ExpMode {
        EXP_DEFAULT,
        EXP_PADE,
        EXP_TAYLOR,
        EXP_PRIME,
    }

    enum LogMode {
        LOG_DEFAULT,
        LOG_PADE,
        LOG_NEWTON,
        LOG_MINMAX,
    }

    enum SigmoidMode {
        SIGMOID_DEFAULT,
        SIGMOID_MM1,
        SIGMOID_SEG3,
        SIGMOID_REAL,
    }

    enum BeaverType {
        TrustedFirstParty,
        TrustedThirdParty,
        MultiParty,
    }

    unsafe extern "C++" {
        include!("libspu/spu.h");
        include!("yacl/link/context.h");
        include!("libspu/device/io.h");
        include!("libspu/core/value.h");
        include!("libspu/core/pt_buffer_view.h");

        type SPUContext;
        type Context;
        type IoClient;
        type Value;
        type PtBufferView;

        fn new_spu_context(config: &RuntimeConfig, lctx: SharedPtr<Context>) -> UniquePtr<SPUContext>;
        fn new_runtime_config(protocol: ProtocolKind, field: FieldType, fxp_fraction_bits: i64) -> UniquePtr<RuntimeConfig>;

        // Link context
        type Party;
        type ContextDesc;

        fn new_context_desc() -> UniquePtr<ContextDesc>;
        fn add_party(desc: &mut UniquePtr<ContextDesc>, id: &str, host: &str);
        fn create_mem_link_context(desc: &UniquePtr<ContextDesc>, self_rank: usize) -> SharedPtr<Context>;

        // IO
        fn new_io_client(world_size: usize, config: &RuntimeConfig) -> UniquePtr<IoClient>;
        fn make_shares(io: &IoClient, bv: &PtBufferView, vtype: Visibility, owner_rank: i32) -> Vec<Value>;
        fn new_pt_buffer_view(ptr: *const c_void, pt_type: PtType, shape: &Vec<i64>, strides: &Vec<i64>) -> UniquePtr<PtBufferView>;
    }
}
