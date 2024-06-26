use pytools::opcode;

pub const OPCODE_MAPPING: [opcode::Opcode; 256] = [
    opcode::Opcode::CACHE,
    opcode::Opcode::BINARY_OP_ADAPTIVE,
    opcode::Opcode::BINARY_OP_ADD_FLOAT,
    opcode::Opcode::RETURN_VALUE,
    opcode::Opcode::BINARY_OP_ADD_INT,
    opcode::Opcode::UNARY_INVERT,
    opcode::Opcode::BINARY_OP_ADD_UNICODE,
    opcode::Opcode::UNARY_NOT,
    opcode::Opcode::BINARY_OP_INPLACE_ADD_UNICODE,
    opcode::Opcode::BINARY_OP_MULTIPLY_FLOAT,
    opcode::Opcode::BINARY_OP_MULTIPLY_INT,
    opcode::Opcode::BINARY_OP_SUBTRACT_FLOAT,
    opcode::Opcode::BINARY_OP_SUBTRACT_INT,
    opcode::Opcode::BINARY_SUBSCR_ADAPTIVE,
    opcode::Opcode::POP_TOP,
    opcode::Opcode::BINARY_SUBSCR_DICT,
    opcode::Opcode::GET_ANEXT,
    opcode::Opcode::GET_ITER,
    opcode::Opcode::LOAD_BUILD_CLASS,
    opcode::Opcode::BINARY_SUBSCR_GETITEM,
    opcode::Opcode::BINARY_SUBSCR_LIST_INT,
    opcode::Opcode::BINARY_SUBSCR_TUPLE_INT,
    opcode::Opcode::LOAD_ASSERTION_ERROR,
    opcode::Opcode::CALL_ADAPTIVE,
    opcode::Opcode::CALL_PY_EXACT_ARGS,
    opcode::Opcode::MATCH_KEYS,
    opcode::Opcode::GET_AITER,
    opcode::Opcode::SETUP_ANNOTATIONS,
    opcode::Opcode::CALL_PY_WITH_DEFAULTS,
    opcode::Opcode::IMPORT_STAR,
    opcode::Opcode::STORE_SUBSCR,
    opcode::Opcode::COMPARE_OP_ADAPTIVE,
    opcode::Opcode::GET_YIELD_FROM_ITER,
    opcode::Opcode::COMPARE_OP_FLOAT_JUMP,
    opcode::Opcode::COMPARE_OP_INT_JUMP,
    opcode::Opcode::COMPARE_OP_STR_JUMP,
    opcode::Opcode::EXTENDED_ARG_QUICK,
    opcode::Opcode::UNARY_NEGATIVE,
    opcode::Opcode::JUMP_BACKWARD_QUICK,
    opcode::Opcode::LOAD_ATTR_ADAPTIVE,
    opcode::Opcode::LOAD_ATTR_INSTANCE_VALUE,
    opcode::Opcode::LOAD_ATTR_MODULE,
    opcode::Opcode::LOAD_ATTR_SLOT,
    opcode::Opcode::POP_EXCEPT,
    opcode::Opcode::LOAD_ATTR_WITH_HINT,
    opcode::Opcode::UNARY_POSITIVE,
    opcode::Opcode::LOAD_CONST__LOAD_FAST,
    opcode::Opcode::LOAD_FAST__LOAD_CONST,
    opcode::Opcode::LOAD_FAST__LOAD_FAST,
    opcode::Opcode::LOAD_GLOBAL_ADAPTIVE,
    opcode::Opcode::CHECK_EXC_MATCH,
    opcode::Opcode::LOAD_GLOBAL_BUILTIN,
    opcode::Opcode::END_ASYNC_FOR,
    opcode::Opcode::LOAD_GLOBAL_MODULE,
    opcode::Opcode::GET_LEN,
    opcode::Opcode::BEFORE_ASYNC_WITH,
    opcode::Opcode::LOAD_METHOD_ADAPTIVE,
    opcode::Opcode::LOAD_METHOD_CLASS,
    opcode::Opcode::LOAD_METHOD_MODULE,
    opcode::Opcode::LOAD_METHOD_NO_DICT,
    opcode::Opcode::RETURN_GENERATOR,
    opcode::Opcode::PUSH_NULL,
    opcode::Opcode::LOAD_METHOD_WITH_DICT,
    opcode::Opcode::BINARY_SUBSCR,
    opcode::Opcode::PUSH_EXC_INFO,
    opcode::Opcode::LOAD_METHOD_WITH_VALUES,
    opcode::Opcode::PRECALL_ADAPTIVE,
    opcode::Opcode::PRECALL_BOUND_METHOD,
    opcode::Opcode::PRECALL_BUILTIN_CLASS,
    opcode::Opcode::YIELD_VALUE,
    opcode::Opcode::PRECALL_BUILTIN_FAST_WITH_KEYWORDS,
    opcode::Opcode::PRECALL_METHOD_DESCRIPTOR_FAST_WITH_KEYWORDS,
    opcode::Opcode::LIST_TO_TUPLE,
    opcode::Opcode::MATCH_MAPPING,
    opcode::Opcode::CHECK_EG_MATCH,
    opcode::Opcode::PRECALL_NO_KW_BUILTIN_FAST,
    opcode::Opcode::PRECALL_NO_KW_BUILTIN_O,
    opcode::Opcode::PRECALL_NO_KW_ISINSTANCE,
    opcode::Opcode::PRECALL_NO_KW_LEN,
    opcode::Opcode::ASYNC_GEN_WRAP,
    opcode::Opcode::DELETE_SUBSCR,
    opcode::Opcode::PRINT_EXPR,
    opcode::Opcode::PRECALL_NO_KW_LIST_APPEND,
    opcode::Opcode::MATCH_SEQUENCE,
    opcode::Opcode::PRECALL_NO_KW_METHOD_DESCRIPTOR_FAST,
    opcode::Opcode::NOP,
    opcode::Opcode::PRECALL_NO_KW_METHOD_DESCRIPTOR_NOARGS,
    opcode::Opcode::BEFORE_WITH,
    opcode::Opcode::WITH_EXCEPT_START,
    opcode::Opcode::PREP_RERAISE_STAR,
    opcode::Opcode::PRECALL_NO_KW_METHOD_DESCRIPTOR_O,
    opcode::Opcode::PRECALL_NO_KW_STR_1,
    opcode::Opcode::PRECALL_NO_KW_TUPLE_1,
    opcode::Opcode::PRECALL_NO_KW_TYPE_1,
    opcode::Opcode::PRECALL_PYFUNC,
    opcode::Opcode::RESUME_QUICK,
    opcode::Opcode::STORE_ATTR_ADAPTIVE,
    opcode::Opcode::STORE_ATTR_INSTANCE_VALUE,
    opcode::Opcode::STORE_ATTR_SLOT,
    opcode::Opcode::STORE_ATTR_WITH_HINT,
    opcode::Opcode::LOAD_CLASSDEREF,
    opcode::Opcode::STORE_FAST__LOAD_FAST,
    opcode::Opcode::LOAD_CLOSURE,
    opcode::Opcode::JUMP_BACKWARD_NO_INTERRUPT,
    opcode::Opcode::STORE_FAST__STORE_FAST,
    opcode::Opcode::STORE_SUBSCR_ADAPTIVE,
    opcode::Opcode::BUILD_STRING,
    opcode::Opcode::STORE_SUBSCR_DICT,
    opcode::Opcode::STORE_GLOBAL,
    opcode::Opcode::COPY_FREE_VARS,
    opcode::Opcode::POP_JUMP_BACKWARD_IF_NONE,
    opcode::Opcode::STORE_SUBSCR_LIST_INT,
    opcode::Opcode::DELETE_ATTR,
    opcode::Opcode::UNPACK_SEQUENCE_ADAPTIVE,
    opcode::Opcode::UNPACK_SEQUENCE_LIST,
    opcode::Opcode::POP_JUMP_FORWARD_IF_NONE,
    opcode::Opcode::UNPACK_SEQUENCE_TUPLE,
    opcode::Opcode::UNPACK_SEQUENCE_TWO_TUPLE,
    opcode::Opcode::BINARY_OP,
    opcode::Opcode::LOAD_FAST,
    opcode::Opcode::FORMAT_VALUE,
    opcode::Opcode::DICT_UPDATE,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::POP_JUMP_BACKWARD_IF_TRUE,
    opcode::Opcode::KW_NAMES,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::STORE_FAST,
    opcode::Opcode::POP_JUMP_FORWARD_IF_FALSE,
    opcode::Opcode::DICT_MERGE,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::EXTENDED_ARG,
    opcode::Opcode::BUILD_SET,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::IMPORT_NAME,
    opcode::Opcode::SEND,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::DELETE_GLOBAL,
    opcode::Opcode::JUMP_IF_TRUE_OR_POP,
    opcode::Opcode::DELETE_NAME,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::POP_JUMP_BACKWARD_IF_FALSE,
    opcode::Opcode::IS_OP,
    opcode::Opcode::POP_JUMP_FORWARD_IF_NOT_NONE,
    opcode::Opcode::MAKE_FUNCTION,
    opcode::Opcode::LIST_EXTEND,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::BUILD_CONST_KEY_MAP,
    opcode::Opcode::POP_JUMP_FORWARD_IF_TRUE,
    opcode::Opcode::SET_UPDATE,
    opcode::Opcode::STORE_ATTR,
    opcode::Opcode::RAISE_VARARGS,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::SET_ADD,
    opcode::Opcode::IMPORT_FROM,
    opcode::Opcode::COMPARE_OP,
    opcode::Opcode::LOAD_ATTR,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::JUMP_FORWARD,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::LIST_APPEND,
    opcode::Opcode::UNPACK_EX,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::GET_AWAITABLE,
    opcode::Opcode::BUILD_LIST,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::CALL,
    opcode::Opcode::CALL_FUNCTION_EX,
    opcode::Opcode::MAKE_CELL,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::SWAP,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::FOR_ITER,
    opcode::Opcode::STORE_NAME,
    opcode::Opcode::COPY,
    opcode::Opcode::MATCH_CLASS,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::LOAD_METHOD,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::JUMP_IF_FALSE_OR_POP,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNPACK_SEQUENCE,
    opcode::Opcode::POP_JUMP_BACKWARD_IF_NOT_NONE,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::RERAISE,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::DELETE_FAST,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::LOAD_CONST,
    opcode::Opcode::RESUME,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::BUILD_SLICE,
    opcode::Opcode::PRECALL,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::JUMP_BACKWARD,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::LOAD_DEREF,
    opcode::Opcode::STORE_DEREF,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::BUILD_TUPLE,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::DELETE_DEREF,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::LOAD_GLOBAL,
    opcode::Opcode::BUILD_MAP,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::CONTAINS_OP,
    opcode::Opcode::MAP_ADD,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::LOAD_NAME,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
    opcode::Opcode::UNKNOWN,
];
