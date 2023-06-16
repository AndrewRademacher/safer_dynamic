/*! \file */
/*******************************************
 *                                         *
 *  File auto-generated by `::safer_ffi`.  *
 *                                         *
 *  Do not manually edit this file.        *
 *                                         *
 *******************************************/

#ifndef __RUST_DYNOMATH__
#define __RUST_DYNOMATH__
#ifdef __cplusplus
extern "C" {
#endif


#include <stddef.h>
#include <stdint.h>

/** <No documentation available> */
int64_t
DYNO_add (
    int64_t a,
    int64_t b);

/** <No documentation available> */
typedef struct DynoClient DynoClient_t;

/** \brief
 *  Free the previously created dyno client.
 */
void
DYNO_free_client (
    DynoClient_t * client);

/** <No documentation available> */
/** \remark Has the same ABI as `uint32_t` **/
#ifdef DOXYGEN
typedef
#endif
enum DynoError {
    /** <No documentation available> */
    DYNO_ERROR_OK = 0,
    /** <No documentation available> */
    DYNO_ERROR_RUNTIME_FAILED = 1,
    /** <No documentation available> */
    DYNO_ERROR_FAILED_TO_CONNECT = 2,
    /** <No documentation available> */
    DYNO_ERROR_FAILED_TO_RECEIVE_BODY = 3,
}
#ifndef DOXYGEN
; typedef uint32_t
#endif
DynoError_t;

/** <No documentation available> */
typedef struct Response {
    /** <No documentation available> */
    DynoError_t error;

    /** <No documentation available> */
    int16_t status_code;

    /** <No documentation available> */
    char * text;
} Response_t;

/** <No documentation available> */
void
DYNO_free_response (
    Response_t response);

/** <No documentation available> */
DynoClient_t *
DYNO_new_client (void);

/** <No documentation available> */
Response_t
DYNO_request (
    DynoClient_t * client,
    char const * url);


#ifdef __cplusplus
} /* extern \"C\" */
#endif

#endif /* __RUST_DYNOMATH__ */