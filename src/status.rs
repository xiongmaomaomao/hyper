#![unstable]
//! Status Codes
use std::fmt;
use std::mem::transmute;

// shamelessly lifted from Teepee. I tried a few schemes, this really
// does seem like the best.

/// An HTTP status code (`Status-Code` in RFC 2616).
///
/// This enum is absolutely exhaustive, covering all 500 possible values (100–599).
///
/// For HTTP/2.0, statuses belonging to the 1xx Informational class are invalid.
///
/// As this is a C‐style enum with each variant having a corresponding value, you may use the likes
/// of `Continue as u16` to retreive the value `100u16`. Normally, though, you should not need to do
/// any such thing; just use the status code as a `StatusCode`.
///
/// If you encounter a status code that you do not know how to deal with, you should treat it as the
/// `x00` status code—e.g. for code 123, treat it as 100 (Continue). This can be achieved with
/// `self.class().default_code()`:
///
/// ```rust
/// # use hyper::status::StatusCode::{Code123, Continue};
/// assert_eq!(Code123.class().default_code(), Continue);
/// ```
#[unstable]
pub enum StatusCode {
    /// 100 Continue
    #[unstable]
    Continue = 100,
    /// 101 Switching Protocols
    #[unstable]
    SwitchingProtocols = 101,
    /// 102 Processing
    #[unstable]
    Processing = 102,
    /// 103 (unregistered)
    #[unstable]
    Code103 = 103,
    /// 104 (unregistered)
    #[unstable]
    Code104 = 104,
    /// 105 (unregistered)
    #[unstable]
    Code105 = 105,
    /// 106 (unregistered)
    #[unstable]
    Code106 = 106,
    /// 107 (unregistered)
    #[unstable]
    Code107 = 107,
    /// 108 (unregistered)
    #[unstable]
    Code108 = 108,
    /// 109 (unregistered)
    #[unstable]
    Code109 = 109,
    /// 110 (unregistered)
    #[unstable]
    Code110 = 110,
    /// 111 (unregistered)
    #[unstable]
    Code111 = 111,
    /// 112 (unregistered)
    #[unstable]
    Code112 = 112,
    /// 113 (unregistered)
    #[unstable]
    Code113 = 113,
    /// 114 (unregistered)
    #[unstable]
    Code114 = 114,
    /// 115 (unregistered)
    #[unstable]
    Code115 = 115,
    /// 116 (unregistered)
    #[unstable]
    Code116 = 116,
    /// 117 (unregistered)
    #[unstable]
    Code117 = 117,
    /// 118 (unregistered)
    #[unstable]
    Code118 = 118,
    /// 119 (unregistered)
    #[unstable]
    Code119 = 119,
    /// 120 (unregistered)
    #[unstable]
    Code120 = 120,
    /// 121 (unregistered)
    #[unstable]
    Code121 = 121,
    /// 122 (unregistered)
    #[unstable]
    Code122 = 122,
    /// 123 (unregistered)
    #[unstable]
    Code123 = 123,
    /// 124 (unregistered)
    #[unstable]
    Code124 = 124,
    /// 125 (unregistered)
    #[unstable]
    Code125 = 125,
    /// 126 (unregistered)
    #[unstable]
    Code126 = 126,
    /// 127 (unregistered)
    #[unstable]
    Code127 = 127,
    /// 128 (unregistered)
    #[unstable]
    Code128 = 128,
    /// 129 (unregistered)
    #[unstable]
    Code129 = 129,
    /// 130 (unregistered)
    #[unstable]
    Code130 = 130,
    /// 131 (unregistered)
    #[unstable]
    Code131 = 131,
    /// 132 (unregistered)
    #[unstable]
    Code132 = 132,
    /// 133 (unregistered)
    #[unstable]
    Code133 = 133,
    /// 134 (unregistered)
    #[unstable]
    Code134 = 134,
    /// 135 (unregistered)
    #[unstable]
    Code135 = 135,
    /// 136 (unregistered)
    #[unstable]
    Code136 = 136,
    /// 137 (unregistered)
    #[unstable]
    Code137 = 137,
    /// 138 (unregistered)
    #[unstable]
    Code138 = 138,
    /// 139 (unregistered)
    #[unstable]
    Code139 = 139,
    /// 140 (unregistered)
    #[unstable]
    Code140 = 140,
    /// 141 (unregistered)
    #[unstable]
    Code141 = 141,
    /// 142 (unregistered)
    #[unstable]
    Code142 = 142,
    /// 143 (unregistered)
    #[unstable]
    Code143 = 143,
    /// 144 (unregistered)
    #[unstable]
    Code144 = 144,
    /// 145 (unregistered)
    #[unstable]
    Code145 = 145,
    /// 146 (unregistered)
    #[unstable]
    Code146 = 146,
    /// 147 (unregistered)
    #[unstable]
    Code147 = 147,
    /// 148 (unregistered)
    #[unstable]
    Code148 = 148,
    /// 149 (unregistered)
    #[unstable]
    Code149 = 149,
    /// 150 (unregistered)
    #[unstable]
    Code150 = 150,
    /// 151 (unregistered)
    #[unstable]
    Code151 = 151,
    /// 152 (unregistered)
    #[unstable]
    Code152 = 152,
    /// 153 (unregistered)
    #[unstable]
    Code153 = 153,
    /// 154 (unregistered)
    #[unstable]
    Code154 = 154,
    /// 155 (unregistered)
    #[unstable]
    Code155 = 155,
    /// 156 (unregistered)
    #[unstable]
    Code156 = 156,
    /// 157 (unregistered)
    #[unstable]
    Code157 = 157,
    /// 158 (unregistered)
    #[unstable]
    Code158 = 158,
    /// 159 (unregistered)
    #[unstable]
    Code159 = 159,
    /// 160 (unregistered)
    #[unstable]
    Code160 = 160,
    /// 161 (unregistered)
    #[unstable]
    Code161 = 161,
    /// 162 (unregistered)
    #[unstable]
    Code162 = 162,
    /// 163 (unregistered)
    #[unstable]
    Code163 = 163,
    /// 164 (unregistered)
    #[unstable]
    Code164 = 164,
    /// 165 (unregistered)
    #[unstable]
    Code165 = 165,
    /// 166 (unregistered)
    #[unstable]
    Code166 = 166,
    /// 167 (unregistered)
    #[unstable]
    Code167 = 167,
    /// 168 (unregistered)
    #[unstable]
    Code168 = 168,
    /// 169 (unregistered)
    #[unstable]
    Code169 = 169,
    /// 170 (unregistered)
    #[unstable]
    Code170 = 170,
    /// 171 (unregistered)
    #[unstable]
    Code171 = 171,
    /// 172 (unregistered)
    #[unstable]
    Code172 = 172,
    /// 173 (unregistered)
    #[unstable]
    Code173 = 173,
    /// 174 (unregistered)
    #[unstable]
    Code174 = 174,
    /// 175 (unregistered)
    #[unstable]
    Code175 = 175,
    /// 176 (unregistered)
    #[unstable]
    Code176 = 176,
    /// 177 (unregistered)
    #[unstable]
    Code177 = 177,
    /// 178 (unregistered)
    #[unstable]
    Code178 = 178,
    /// 179 (unregistered)
    #[unstable]
    Code179 = 179,
    /// 180 (unregistered)
    #[unstable]
    Code180 = 180,
    /// 181 (unregistered)
    #[unstable]
    Code181 = 181,
    /// 182 (unregistered)
    #[unstable]
    Code182 = 182,
    /// 183 (unregistered)
    #[unstable]
    Code183 = 183,
    /// 184 (unregistered)
    #[unstable]
    Code184 = 184,
    /// 185 (unregistered)
    #[unstable]
    Code185 = 185,
    /// 186 (unregistered)
    #[unstable]
    Code186 = 186,
    /// 187 (unregistered)
    #[unstable]
    Code187 = 187,
    /// 188 (unregistered)
    #[unstable]
    Code188 = 188,
    /// 189 (unregistered)
    #[unstable]
    Code189 = 189,
    /// 190 (unregistered)
    #[unstable]
    Code190 = 190,
    /// 191 (unregistered)
    #[unstable]
    Code191 = 191,
    /// 192 (unregistered)
    #[unstable]
    Code192 = 192,
    /// 193 (unregistered)
    #[unstable]
    Code193 = 193,
    /// 194 (unregistered)
    #[unstable]
    Code194 = 194,
    /// 195 (unregistered)
    #[unstable]
    Code195 = 195,
    /// 196 (unregistered)
    #[unstable]
    Code196 = 196,
    /// 197 (unregistered)
    #[unstable]
    Code197 = 197,
    /// 198 (unregistered)
    #[unstable]
    Code198 = 198,
    /// 199 (unregistered)
    #[unstable]
    Code199 = 199,

    /// 200 OK
    #[unstable]
    Ok = 200,
    /// 201 Created
    #[unstable]
    Created = 201,
    /// 202 Accepted
    #[unstable]
    Accepted = 202,
    /// 203 Non-Authoritative Information
    #[unstable]
    NonAuthoritativeInformation = 203,
    /// 204 No Content
    #[unstable]
    NoContent = 204,
    /// 205 Reset Content
    #[unstable]
    ResetContent = 205,
    /// 206 Partial Content
    #[unstable]
    PartialContent = 206,
    /// 207 Multi-Status
    #[unstable]
    MultiStatus = 207,
    /// 208 Already Reported
    #[unstable]
    AlreadyReported = 208,
    /// 209 (unregistered)
    #[unstable]
    Code209 = 209,
    /// 210 (unregistered)
    #[unstable]
    Code210 = 210,
    /// 211 (unregistered)
    #[unstable]
    Code211 = 211,
    /// 212 (unregistered)
    #[unstable]
    Code212 = 212,
    /// 213 (unregistered)
    #[unstable]
    Code213 = 213,
    /// 214 (unregistered)
    #[unstable]
    Code214 = 214,
    /// 215 (unregistered)
    #[unstable]
    Code215 = 215,
    /// 216 (unregistered)
    #[unstable]
    Code216 = 216,
    /// 217 (unregistered)
    #[unstable]
    Code217 = 217,
    /// 218 (unregistered)
    #[unstable]
    Code218 = 218,
    /// 219 (unregistered)
    #[unstable]
    Code219 = 219,
    /// 220 (unregistered)
    #[unstable]
    Code220 = 220,
    /// 221 (unregistered)
    #[unstable]
    Code221 = 221,
    /// 222 (unregistered)
    #[unstable]
    Code222 = 222,
    /// 223 (unregistered)
    #[unstable]
    Code223 = 223,
    /// 224 (unregistered)
    #[unstable]
    Code224 = 224,
    /// 225 (unregistered)
    #[unstable]
    Code225 = 225,
    /// 226 IM Used
    #[unstable]
    ImUsed = 226,
    /// 227 (unregistered)
    #[unstable]
    Code227 = 227,
    /// 228 (unregistered)
    #[unstable]
    Code228 = 228,
    /// 229 (unregistered)
    #[unstable]
    Code229 = 229,
    /// 230 (unregistered)
    #[unstable]
    Code230 = 230,
    /// 231 (unregistered)
    #[unstable]
    Code231 = 231,
    /// 232 (unregistered)
    #[unstable]
    Code232 = 232,
    /// 233 (unregistered)
    #[unstable]
    Code233 = 233,
    /// 234 (unregistered)
    #[unstable]
    Code234 = 234,
    /// 235 (unregistered)
    #[unstable]
    Code235 = 235,
    /// 236 (unregistered)
    #[unstable]
    Code236 = 236,
    /// 237 (unregistered)
    #[unstable]
    Code237 = 237,
    /// 238 (unregistered)
    #[unstable]
    Code238 = 238,
    /// 239 (unregistered)
    #[unstable]
    Code239 = 239,
    /// 240 (unregistered)
    #[unstable]
    Code240 = 240,
    /// 241 (unregistered)
    #[unstable]
    Code241 = 241,
    /// 242 (unregistered)
    #[unstable]
    Code242 = 242,
    /// 243 (unregistered)
    #[unstable]
    Code243 = 243,
    /// 244 (unregistered)
    #[unstable]
    Code244 = 244,
    /// 245 (unregistered)
    #[unstable]
    Code245 = 245,
    /// 246 (unregistered)
    #[unstable]
    Code246 = 246,
    /// 247 (unregistered)
    #[unstable]
    Code247 = 247,
    /// 248 (unregistered)
    #[unstable]
    Code248 = 248,
    /// 249 (unregistered)
    #[unstable]
    Code249 = 249,
    /// 250 (unregistered)
    #[unstable]
    Code250 = 250,
    /// 251 (unregistered)
    #[unstable]
    Code251 = 251,
    /// 252 (unregistered)
    #[unstable]
    Code252 = 252,
    /// 253 (unregistered)
    #[unstable]
    Code253 = 253,
    /// 254 (unregistered)
    #[unstable]
    Code254 = 254,
    /// 255 (unregistered)
    #[unstable]
    Code255 = 255,
    /// 256 (unregistered)
    #[unstable]
    Code256 = 256,
    /// 257 (unregistered)
    #[unstable]
    Code257 = 257,
    /// 258 (unregistered)
    #[unstable]
    Code258 = 258,
    /// 259 (unregistered)
    #[unstable]
    Code259 = 259,
    /// 260 (unregistered)
    #[unstable]
    Code260 = 260,
    /// 261 (unregistered)
    #[unstable]
    Code261 = 261,
    /// 262 (unregistered)
    #[unstable]
    Code262 = 262,
    /// 263 (unregistered)
    #[unstable]
    Code263 = 263,
    /// 264 (unregistered)
    #[unstable]
    Code264 = 264,
    /// 265 (unregistered)
    #[unstable]
    Code265 = 265,
    /// 266 (unregistered)
    #[unstable]
    Code266 = 266,
    /// 267 (unregistered)
    #[unstable]
    Code267 = 267,
    /// 268 (unregistered)
    #[unstable]
    Code268 = 268,
    /// 269 (unregistered)
    #[unstable]
    Code269 = 269,
    /// 270 (unregistered)
    #[unstable]
    Code270 = 270,
    /// 271 (unregistered)
    #[unstable]
    Code271 = 271,
    /// 272 (unregistered)
    #[unstable]
    Code272 = 272,
    /// 273 (unregistered)
    #[unstable]
    Code273 = 273,
    /// 274 (unregistered)
    #[unstable]
    Code274 = 274,
    /// 275 (unregistered)
    #[unstable]
    Code275 = 275,
    /// 276 (unregistered)
    #[unstable]
    Code276 = 276,
    /// 277 (unregistered)
    #[unstable]
    Code277 = 277,
    /// 278 (unregistered)
    #[unstable]
    Code278 = 278,
    /// 279 (unregistered)
    #[unstable]
    Code279 = 279,
    /// 280 (unregistered)
    #[unstable]
    Code280 = 280,
    /// 281 (unregistered)
    #[unstable]
    Code281 = 281,
    /// 282 (unregistered)
    #[unstable]
    Code282 = 282,
    /// 283 (unregistered)
    #[unstable]
    Code283 = 283,
    /// 284 (unregistered)
    #[unstable]
    Code284 = 284,
    /// 285 (unregistered)
    #[unstable]
    Code285 = 285,
    /// 286 (unregistered)
    #[unstable]
    Code286 = 286,
    /// 287 (unregistered)
    #[unstable]
    Code287 = 287,
    /// 288 (unregistered)
    #[unstable]
    Code288 = 288,
    /// 289 (unregistered)
    #[unstable]
    Code289 = 289,
    /// 290 (unregistered)
    #[unstable]
    Code290 = 290,
    /// 291 (unregistered)
    #[unstable]
    Code291 = 291,
    /// 292 (unregistered)
    #[unstable]
    Code292 = 292,
    /// 293 (unregistered)
    #[unstable]
    Code293 = 293,
    /// 294 (unregistered)
    #[unstable]
    Code294 = 294,
    /// 295 (unregistered)
    #[unstable]
    Code295 = 295,
    /// 296 (unregistered)
    #[unstable]
    Code296 = 296,
    /// 297 (unregistered)
    #[unstable]
    Code297 = 297,
    /// 298 (unregistered)
    #[unstable]
    Code298 = 298,
    /// 299 (unregistered)
    #[unstable]
    Code299 = 299,

    /// 300 Multiple Choices
    #[unstable]
    MultipleChoices = 300,
    /// 301 Moved Permanently
    #[unstable]
    MovedPermanently = 301,
    /// 302 Found
    #[unstable]
    Found = 302,
    /// 303 See Other
    #[unstable]
    SeeOther = 303,
    /// 304 Not Modified
    #[unstable]
    NotModified = 304,
    /// 305 Use Proxy
    #[unstable]
    UseProxy = 305,
    /// 306 Switch Proxy
    #[unstable]
    SwitchProxy = 306,
    /// 307 Temporary Redirect
    #[unstable]
    TemporaryRedirect = 307,
    /// 308 Permanent Redirect
    #[unstable]
    PermanentRedirect = 308,
    /// 309 (unregistered)
    #[unstable]
    Code309 = 309,
    /// 310 (unregistered)
    #[unstable]
    Code310 = 310,
    /// 311 (unregistered)
    #[unstable]
    Code311 = 311,
    /// 312 (unregistered)
    #[unstable]
    Code312 = 312,
    /// 313 (unregistered)
    #[unstable]
    Code313 = 313,
    /// 314 (unregistered)
    #[unstable]
    Code314 = 314,
    /// 315 (unregistered)
    #[unstable]
    Code315 = 315,
    /// 316 (unregistered)
    #[unstable]
    Code316 = 316,
    /// 317 (unregistered)
    #[unstable]
    Code317 = 317,
    /// 318 (unregistered)
    #[unstable]
    Code318 = 318,
    /// 319 (unregistered)
    #[unstable]
    Code319 = 319,
    /// 320 (unregistered)
    #[unstable]
    Code320 = 320,
    /// 321 (unregistered)
    #[unstable]
    Code321 = 321,
    /// 322 (unregistered)
    #[unstable]
    Code322 = 322,
    /// 323 (unregistered)
    #[unstable]
    Code323 = 323,
    /// 324 (unregistered)
    #[unstable]
    Code324 = 324,
    /// 325 (unregistered)
    #[unstable]
    Code325 = 325,
    /// 326 (unregistered)
    #[unstable]
    Code326 = 326,
    /// 327 (unregistered)
    #[unstable]
    Code327 = 327,
    /// 328 (unregistered)
    #[unstable]
    Code328 = 328,
    /// 329 (unregistered)
    #[unstable]
    Code329 = 329,
    /// 330 (unregistered)
    #[unstable]
    Code330 = 330,
    /// 331 (unregistered)
    #[unstable]
    Code331 = 331,
    /// 332 (unregistered)
    #[unstable]
    Code332 = 332,
    /// 333 (unregistered)
    #[unstable]
    Code333 = 333,
    /// 334 (unregistered)
    #[unstable]
    Code334 = 334,
    /// 335 (unregistered)
    #[unstable]
    Code335 = 335,
    /// 336 (unregistered)
    #[unstable]
    Code336 = 336,
    /// 337 (unregistered)
    #[unstable]
    Code337 = 337,
    /// 338 (unregistered)
    #[unstable]
    Code338 = 338,
    /// 339 (unregistered)
    #[unstable]
    Code339 = 339,
    /// 340 (unregistered)
    #[unstable]
    Code340 = 340,
    /// 341 (unregistered)
    #[unstable]
    Code341 = 341,
    /// 342 (unregistered)
    #[unstable]
    Code342 = 342,
    /// 343 (unregistered)
    #[unstable]
    Code343 = 343,
    /// 344 (unregistered)
    #[unstable]
    Code344 = 344,
    /// 345 (unregistered)
    #[unstable]
    Code345 = 345,
    /// 346 (unregistered)
    #[unstable]
    Code346 = 346,
    /// 347 (unregistered)
    #[unstable]
    Code347 = 347,
    /// 348 (unregistered)
    #[unstable]
    Code348 = 348,
    /// 349 (unregistered)
    #[unstable]
    Code349 = 349,
    /// 350 (unregistered)
    #[unstable]
    Code350 = 350,
    /// 351 (unregistered)
    #[unstable]
    Code351 = 351,
    /// 352 (unregistered)
    #[unstable]
    Code352 = 352,
    /// 353 (unregistered)
    #[unstable]
    Code353 = 353,
    /// 354 (unregistered)
    #[unstable]
    Code354 = 354,
    /// 355 (unregistered)
    #[unstable]
    Code355 = 355,
    /// 356 (unregistered)
    #[unstable]
    Code356 = 356,
    /// 357 (unregistered)
    #[unstable]
    Code357 = 357,
    /// 358 (unregistered)
    #[unstable]
    Code358 = 358,
    /// 359 (unregistered)
    #[unstable]
    Code359 = 359,
    /// 360 (unregistered)
    #[unstable]
    Code360 = 360,
    /// 361 (unregistered)
    #[unstable]
    Code361 = 361,
    /// 362 (unregistered)
    #[unstable]
    Code362 = 362,
    /// 363 (unregistered)
    #[unstable]
    Code363 = 363,
    /// 364 (unregistered)
    #[unstable]
    Code364 = 364,
    /// 365 (unregistered)
    #[unstable]
    Code365 = 365,
    /// 366 (unregistered)
    #[unstable]
    Code366 = 366,
    /// 367 (unregistered)
    #[unstable]
    Code367 = 367,
    /// 368 (unregistered)
    #[unstable]
    Code368 = 368,
    /// 369 (unregistered)
    #[unstable]
    Code369 = 369,
    /// 370 (unregistered)
    #[unstable]
    Code370 = 370,
    /// 371 (unregistered)
    #[unstable]
    Code371 = 371,
    /// 372 (unregistered)
    #[unstable]
    Code372 = 372,
    /// 373 (unregistered)
    #[unstable]
    Code373 = 373,
    /// 374 (unregistered)
    #[unstable]
    Code374 = 374,
    /// 375 (unregistered)
    #[unstable]
    Code375 = 375,
    /// 376 (unregistered)
    #[unstable]
    Code376 = 376,
    /// 377 (unregistered)
    #[unstable]
    Code377 = 377,
    /// 378 (unregistered)
    #[unstable]
    Code378 = 378,
    /// 379 (unregistered)
    #[unstable]
    Code379 = 379,
    /// 380 (unregistered)
    #[unstable]
    Code380 = 380,
    /// 381 (unregistered)
    #[unstable]
    Code381 = 381,
    /// 382 (unregistered)
    #[unstable]
    Code382 = 382,
    /// 383 (unregistered)
    #[unstable]
    Code383 = 383,
    /// 384 (unregistered)
    #[unstable]
    Code384 = 384,
    /// 385 (unregistered)
    #[unstable]
    Code385 = 385,
    /// 386 (unregistered)
    #[unstable]
    Code386 = 386,
    /// 387 (unregistered)
    #[unstable]
    Code387 = 387,
    /// 388 (unregistered)
    #[unstable]
    Code388 = 388,
    /// 389 (unregistered)
    #[unstable]
    Code389 = 389,
    /// 390 (unregistered)
    #[unstable]
    Code390 = 390,
    /// 391 (unregistered)
    #[unstable]
    Code391 = 391,
    /// 392 (unregistered)
    #[unstable]
    Code392 = 392,
    /// 393 (unregistered)
    #[unstable]
    Code393 = 393,
    /// 394 (unregistered)
    #[unstable]
    Code394 = 394,
    /// 395 (unregistered)
    #[unstable]
    Code395 = 395,
    /// 396 (unregistered)
    #[unstable]
    Code396 = 396,
    /// 397 (unregistered)
    #[unstable]
    Code397 = 397,
    /// 398 (unregistered)
    #[unstable]
    Code398 = 398,
    /// 399 (unregistered)
    #[unstable]
    Code399 = 399,

    /// 400 Bad Request
    #[unstable]
    BadRequest = 400,
    /// 401 Unauthorized
    #[unstable]
    Unauthorized = 401,
    /// 402 Payment Required
    #[unstable]
    PaymentRequired = 402,
    /// 403 Forbidden
    #[unstable]
    Forbidden = 403,
    /// 404 Not Found
    #[unstable]
    NotFound = 404,
    /// 405 Method Not Allowed
    #[unstable]
    MethodNotAllowed = 405,
    /// 406 Not Acceptable
    #[unstable]
    NotAcceptable = 406,
    /// 407 Proxy Authentication Required
    #[unstable]
    ProxyAuthenticationRequired = 407,
    /// 408 Request Timeout
    #[unstable]
    RequestTimeout = 408,
    /// 409 Conflict
    #[unstable]
    Conflict = 409,
    /// 410 Gone
    #[unstable]
    Gone = 410,
    /// 411 Length Required
    #[unstable]
    LengthRequired = 411,
    /// 412 Precondition Failed
    #[unstable]
    PreconditionFailed = 412,
    /// 413 Request Entity Too Large
    #[unstable]
    RequestEntityTooLarge = 413,
    /// 414 Request-URI Too Long
    #[unstable]
    RequestUriTooLong = 414,
    /// 415 Unsupported Media Type
    #[unstable]
    UnsupportedMediaType = 415,
    /// 416 Requested Range Not Satisfiable
    #[unstable]
    RequestedRangeNotSatisfiable = 416,
    /// 417 Expectation Failed
    #[unstable]
    ExpectationFailed = 417,
    /// 418 I'm a teapot
    #[unstable]
    ImATeapot = 418,
    /// 419 Authentication Timeout
    #[unstable]
    AuthenticationTimeout = 419,
    /// 420 (unregistered)
    #[unstable]
    Code420 = 420,
    /// 421 (unregistered)
    #[unstable]
    Code421 = 421,
    /// 422 Unprocessable Entity
    #[unstable]
    UnprocessableEntity = 422,
    /// 423 Locked
    #[unstable]
    Locked = 423,
    /// 424 Failed Dependency
    #[unstable]
    FailedDependency = 424,
    /// 425 Unordered Collection
    #[unstable]
    UnorderedCollection = 425,
    /// 426 Upgrade Required
    #[unstable]
    UpgradeRequired = 426,
    /// 427 (unregistered)
    #[unstable]
    Code427 = 427,
    /// 428 Precondition Required
    #[unstable]
    PreconditionRequired = 428,
    /// 429 Too Many Requests
    #[unstable]
    TooManyRequests = 429,
    /// 430 (unregistered)
    #[unstable]
    Code430 = 430,
    /// 431 Request Header Fields Too Large
    #[unstable]
    RequestHeaderFieldsTooLarge = 431,
    /// 432 (unregistered)
    #[unstable]
    Code432 = 432,
    /// 433 (unregistered)
    #[unstable]
    Code433 = 433,
    /// 434 (unregistered)
    #[unstable]
    Code434 = 434,
    /// 435 (unregistered)
    #[unstable]
    Code435 = 435,
    /// 436 (unregistered)
    #[unstable]
    Code436 = 436,
    /// 437 (unregistered)
    #[unstable]
    Code437 = 437,
    /// 438 (unregistered)
    #[unstable]
    Code438 = 438,
    /// 439 (unregistered)
    #[unstable]
    Code439 = 439,
    /// 440 (unregistered)
    #[unstable]
    Code440 = 440,
    /// 441 (unregistered)
    #[unstable]
    Code441 = 441,
    /// 442 (unregistered)
    #[unstable]
    Code442 = 442,
    /// 443 (unregistered)
    #[unstable]
    Code443 = 443,
    /// 444 (unregistered)
    #[unstable]
    Code444 = 444,
    /// 445 (unregistered)
    #[unstable]
    Code445 = 445,
    /// 446 (unregistered)
    #[unstable]
    Code446 = 446,
    /// 447 (unregistered)
    #[unstable]
    Code447 = 447,
    /// 448 (unregistered)
    #[unstable]
    Code448 = 448,
    /// 449 (unregistered)
    #[unstable]
    Code449 = 449,
    /// 450 (unregistered)
    #[unstable]
    Code450 = 450,
    /// 451 Unavailable For Legal Reasons
    #[unstable]
    UnavailableForLegalReasons = 451,
    /// 452 (unregistered)
    #[unstable]
    Code452 = 452,
    /// 453 (unregistered)
    #[unstable]
    Code453 = 453,
    /// 454 (unregistered)
    #[unstable]
    Code454 = 454,
    /// 455 (unregistered)
    #[unstable]
    Code455 = 455,
    /// 456 (unregistered)
    #[unstable]
    Code456 = 456,
    /// 457 (unregistered)
    #[unstable]
    Code457 = 457,
    /// 458 (unregistered)
    #[unstable]
    Code458 = 458,
    /// 459 (unregistered)
    #[unstable]
    Code459 = 459,
    /// 460 (unregistered)
    #[unstable]
    Code460 = 460,
    /// 461 (unregistered)
    #[unstable]
    Code461 = 461,
    /// 462 (unregistered)
    #[unstable]
    Code462 = 462,
    /// 463 (unregistered)
    #[unstable]
    Code463 = 463,
    /// 464 (unregistered)
    #[unstable]
    Code464 = 464,
    /// 465 (unregistered)
    #[unstable]
    Code465 = 465,
    /// 466 (unregistered)
    #[unstable]
    Code466 = 466,
    /// 467 (unregistered)
    #[unstable]
    Code467 = 467,
    /// 468 (unregistered)
    #[unstable]
    Code468 = 468,
    /// 469 (unregistered)
    #[unstable]
    Code469 = 469,
    /// 470 (unregistered)
    #[unstable]
    Code470 = 470,
    /// 471 (unregistered)
    #[unstable]
    Code471 = 471,
    /// 472 (unregistered)
    #[unstable]
    Code472 = 472,
    /// 473 (unregistered)
    #[unstable]
    Code473 = 473,
    /// 474 (unregistered)
    #[unstable]
    Code474 = 474,
    /// 475 (unregistered)
    #[unstable]
    Code475 = 475,
    /// 476 (unregistered)
    #[unstable]
    Code476 = 476,
    /// 477 (unregistered)
    #[unstable]
    Code477 = 477,
    /// 478 (unregistered)
    #[unstable]
    Code478 = 478,
    /// 479 (unregistered)
    #[unstable]
    Code479 = 479,
    /// 480 (unregistered)
    #[unstable]
    Code480 = 480,
    /// 481 (unregistered)
    #[unstable]
    Code481 = 481,
    /// 482 (unregistered)
    #[unstable]
    Code482 = 482,
    /// 483 (unregistered)
    #[unstable]
    Code483 = 483,
    /// 484 (unregistered)
    #[unstable]
    Code484 = 484,
    /// 485 (unregistered)
    #[unstable]
    Code485 = 485,
    /// 486 (unregistered)
    #[unstable]
    Code486 = 486,
    /// 487 (unregistered)
    #[unstable]
    Code487 = 487,
    /// 488 (unregistered)
    #[unstable]
    Code488 = 488,
    /// 489 (unregistered)
    #[unstable]
    Code489 = 489,
    /// 490 (unregistered)
    #[unstable]
    Code490 = 490,
    /// 491 (unregistered)
    #[unstable]
    Code491 = 491,
    /// 492 (unregistered)
    #[unstable]
    Code492 = 492,
    /// 493 (unregistered)
    #[unstable]
    Code493 = 493,
    /// 494 (unregistered)
    #[unstable]
    Code494 = 494,
    /// 495 (unregistered)
    #[unstable]
    Code495 = 495,
    /// 496 (unregistered)
    #[unstable]
    Code496 = 496,
    /// 497 (unregistered)
    #[unstable]
    Code497 = 497,
    /// 498 (unregistered)
    #[unstable]
    Code498 = 498,
    /// 499 (unregistered)
    #[unstable]
    Code499 = 499,

    /// 500 Internal Server Error
    #[unstable]
    InternalServerError = 500,
    /// 501 Not Implemented
    #[unstable]
    NotImplemented = 501,
    /// 502 Bad Gateway
    #[unstable]
    BadGateway = 502,
    /// 503 Service Unavailable
    #[unstable]
    ServiceUnavailable = 503,
    /// 504 Gateway Timeout
    #[unstable]
    GatewayTimeout = 504,
    /// 505 HTTP Version Not Supported
    #[unstable]
    HttpVersionNotSupported = 505,
    /// 506 Variant Also Negotiates
    #[unstable]
    VariantAlsoNegotiates = 506,
    /// 507 Insufficient Storage
    #[unstable]
    InsufficientStorage = 507,
    /// 508 Loop Detected
    #[unstable]
    LoopDetected = 508,
    /// 509 (unregistered)
    #[unstable]
    Code509 = 509,
    /// 510 Not Extended
    #[unstable]
    NotExtended = 510,
    /// 511 Network Authentication Required
    #[unstable]
    NetworkAuthenticationRequired = 511,
    /// 512 (unregistered)
    #[unstable]
    Code512 = 512,
    /// 513 (unregistered)
    #[unstable]
    Code513 = 513,
    /// 514 (unregistered)
    #[unstable]
    Code514 = 514,
    /// 515 (unregistered)
    #[unstable]
    Code515 = 515,
    /// 516 (unregistered)
    #[unstable]
    Code516 = 516,
    /// 517 (unregistered)
    #[unstable]
    Code517 = 517,
    /// 518 (unregistered)
    #[unstable]
    Code518 = 518,
    /// 519 (unregistered)
    #[unstable]
    Code519 = 519,
    /// 520 (unregistered)
    #[unstable]
    Code520 = 520,
    /// 521 (unregistered)
    #[unstable]
    Code521 = 521,
    /// 522 (unregistered)
    #[unstable]
    Code522 = 522,
    /// 523 (unregistered)
    #[unstable]
    Code523 = 523,
    /// 524 (unregistered)
    #[unstable]
    Code524 = 524,
    /// 525 (unregistered)
    #[unstable]
    Code525 = 525,
    /// 526 (unregistered)
    #[unstable]
    Code526 = 526,
    /// 527 (unregistered)
    #[unstable]
    Code527 = 527,
    /// 528 (unregistered)
    #[unstable]
    Code528 = 528,
    /// 529 (unregistered)
    #[unstable]
    Code529 = 529,
    /// 530 (unregistered)
    #[unstable]
    Code530 = 530,
    /// 531 (unregistered)
    #[unstable]
    Code531 = 531,
    /// 532 (unregistered)
    #[unstable]
    Code532 = 532,
    /// 533 (unregistered)
    #[unstable]
    Code533 = 533,
    /// 534 (unregistered)
    #[unstable]
    Code534 = 534,
    /// 535 (unregistered)
    #[unstable]
    Code535 = 535,
    /// 536 (unregistered)
    #[unstable]
    Code536 = 536,
    /// 537 (unregistered)
    #[unstable]
    Code537 = 537,
    /// 538 (unregistered)
    #[unstable]
    Code538 = 538,
    /// 539 (unregistered)
    #[unstable]
    Code539 = 539,
    /// 540 (unregistered)
    #[unstable]
    Code540 = 540,
    /// 541 (unregistered)
    #[unstable]
    Code541 = 541,
    /// 542 (unregistered)
    #[unstable]
    Code542 = 542,
    /// 543 (unregistered)
    #[unstable]
    Code543 = 543,
    /// 544 (unregistered)
    #[unstable]
    Code544 = 544,
    /// 545 (unregistered)
    #[unstable]
    Code545 = 545,
    /// 546 (unregistered)
    #[unstable]
    Code546 = 546,
    /// 547 (unregistered)
    #[unstable]
    Code547 = 547,
    /// 548 (unregistered)
    #[unstable]
    Code548 = 548,
    /// 549 (unregistered)
    #[unstable]
    Code549 = 549,
    /// 550 (unregistered)
    #[unstable]
    Code550 = 550,
    /// 551 (unregistered)
    #[unstable]
    Code551 = 551,
    /// 552 (unregistered)
    #[unstable]
    Code552 = 552,
    /// 553 (unregistered)
    #[unstable]
    Code553 = 553,
    /// 554 (unregistered)
    #[unstable]
    Code554 = 554,
    /// 555 (unregistered)
    #[unstable]
    Code555 = 555,
    /// 556 (unregistered)
    #[unstable]
    Code556 = 556,
    /// 557 (unregistered)
    #[unstable]
    Code557 = 557,
    /// 558 (unregistered)
    #[unstable]
    Code558 = 558,
    /// 559 (unregistered)
    #[unstable]
    Code559 = 559,
    /// 560 (unregistered)
    #[unstable]
    Code560 = 560,
    /// 561 (unregistered)
    #[unstable]
    Code561 = 561,
    /// 562 (unregistered)
    #[unstable]
    Code562 = 562,
    /// 563 (unregistered)
    #[unstable]
    Code563 = 563,
    /// 564 (unregistered)
    #[unstable]
    Code564 = 564,
    /// 565 (unregistered)
    #[unstable]
    Code565 = 565,
    /// 566 (unregistered)
    #[unstable]
    Code566 = 566,
    /// 567 (unregistered)
    #[unstable]
    Code567 = 567,
    /// 568 (unregistered)
    #[unstable]
    Code568 = 568,
    /// 569 (unregistered)
    #[unstable]
    Code569 = 569,
    /// 570 (unregistered)
    #[unstable]
    Code570 = 570,
    /// 571 (unregistered)
    #[unstable]
    Code571 = 571,
    /// 572 (unregistered)
    #[unstable]
    Code572 = 572,
    /// 573 (unregistered)
    #[unstable]
    Code573 = 573,
    /// 574 (unregistered)
    #[unstable]
    Code574 = 574,
    /// 575 (unregistered)
    #[unstable]
    Code575 = 575,
    /// 576 (unregistered)
    #[unstable]
    Code576 = 576,
    /// 577 (unregistered)
    #[unstable]
    Code577 = 577,
    /// 578 (unregistered)
    #[unstable]
    Code578 = 578,
    /// 579 (unregistered)
    #[unstable]
    Code579 = 579,
    /// 580 (unregistered)
    #[unstable]
    Code580 = 580,
    /// 581 (unregistered)
    #[unstable]
    Code581 = 581,
    /// 582 (unregistered)
    #[unstable]
    Code582 = 582,
    /// 583 (unregistered)
    #[unstable]
    Code583 = 583,
    /// 584 (unregistered)
    #[unstable]
    Code584 = 584,
    /// 585 (unregistered)
    #[unstable]
    Code585 = 585,
    /// 586 (unregistered)
    #[unstable]
    Code586 = 586,
    /// 587 (unregistered)
    #[unstable]
    Code587 = 587,
    /// 588 (unregistered)
    #[unstable]
    Code588 = 588,
    /// 589 (unregistered)
    #[unstable]
    Code589 = 589,
    /// 590 (unregistered)
    #[unstable]
    Code590 = 590,
    /// 591 (unregistered)
    #[unstable]
    Code591 = 591,
    /// 592 (unregistered)
    #[unstable]
    Code592 = 592,
    /// 593 (unregistered)
    #[unstable]
    Code593 = 593,
    /// 594 (unregistered)
    #[unstable]
    Code594 = 594,
    /// 595 (unregistered)
    #[unstable]
    Code595 = 595,
    /// 596 (unregistered)
    #[unstable]
    Code596 = 596,
    /// 597 (unregistered)
    #[unstable]
    Code597 = 597,
    /// 598 (unregistered)
    #[unstable]
    Code598 = 598,
    /// 599 (unregistered)
    #[unstable]
    Code599 = 599,
}

impl StatusCode {

    /// Get the standardised `Reason-Phrase` for this status code.
    ///
    /// This is mostly here for servers writing responses, but could potentially have application at
    /// other times.
    ///
    /// The reason phrase is defined as being exclusively for human readers. You should avoid
    /// deriving any meaning from it at all costs.
    ///
    /// Bear in mind also that in HTTP/2.0 the reason phrase is abolished from transmission, and so
    /// this canonical reason phrase really is the only reason phrase you’ll find.
    #[unstable]
    pub fn canonical_reason(&self) -> Option<&'static str> {
        match *self {
            StatusCode::Continue => Some("Continue"),
            StatusCode::SwitchingProtocols => Some("Switching Protocols"),
            StatusCode::Processing => Some("Processing"),
            StatusCode::Code103 => None,
            StatusCode::Code104 => None,
            StatusCode::Code105 => None,
            StatusCode::Code106 => None,
            StatusCode::Code107 => None,
            StatusCode::Code108 => None,
            StatusCode::Code109 => None,
            StatusCode::Code110 => None,
            StatusCode::Code111 => None,
            StatusCode::Code112 => None,
            StatusCode::Code113 => None,
            StatusCode::Code114 => None,
            StatusCode::Code115 => None,
            StatusCode::Code116 => None,
            StatusCode::Code117 => None,
            StatusCode::Code118 => None,
            StatusCode::Code119 => None,
            StatusCode::Code120 => None,
            StatusCode::Code121 => None,
            StatusCode::Code122 => None,
            StatusCode::Code123 => None,
            StatusCode::Code124 => None,
            StatusCode::Code125 => None,
            StatusCode::Code126 => None,
            StatusCode::Code127 => None,
            StatusCode::Code128 => None,
            StatusCode::Code129 => None,
            StatusCode::Code130 => None,
            StatusCode::Code131 => None,
            StatusCode::Code132 => None,
            StatusCode::Code133 => None,
            StatusCode::Code134 => None,
            StatusCode::Code135 => None,
            StatusCode::Code136 => None,
            StatusCode::Code137 => None,
            StatusCode::Code138 => None,
            StatusCode::Code139 => None,
            StatusCode::Code140 => None,
            StatusCode::Code141 => None,
            StatusCode::Code142 => None,
            StatusCode::Code143 => None,
            StatusCode::Code144 => None,
            StatusCode::Code145 => None,
            StatusCode::Code146 => None,
            StatusCode::Code147 => None,
            StatusCode::Code148 => None,
            StatusCode::Code149 => None,
            StatusCode::Code150 => None,
            StatusCode::Code151 => None,
            StatusCode::Code152 => None,
            StatusCode::Code153 => None,
            StatusCode::Code154 => None,
            StatusCode::Code155 => None,
            StatusCode::Code156 => None,
            StatusCode::Code157 => None,
            StatusCode::Code158 => None,
            StatusCode::Code159 => None,
            StatusCode::Code160 => None,
            StatusCode::Code161 => None,
            StatusCode::Code162 => None,
            StatusCode::Code163 => None,
            StatusCode::Code164 => None,
            StatusCode::Code165 => None,
            StatusCode::Code166 => None,
            StatusCode::Code167 => None,
            StatusCode::Code168 => None,
            StatusCode::Code169 => None,
            StatusCode::Code170 => None,
            StatusCode::Code171 => None,
            StatusCode::Code172 => None,
            StatusCode::Code173 => None,
            StatusCode::Code174 => None,
            StatusCode::Code175 => None,
            StatusCode::Code176 => None,
            StatusCode::Code177 => None,
            StatusCode::Code178 => None,
            StatusCode::Code179 => None,
            StatusCode::Code180 => None,
            StatusCode::Code181 => None,
            StatusCode::Code182 => None,
            StatusCode::Code183 => None,
            StatusCode::Code184 => None,
            StatusCode::Code185 => None,
            StatusCode::Code186 => None,
            StatusCode::Code187 => None,
            StatusCode::Code188 => None,
            StatusCode::Code189 => None,
            StatusCode::Code190 => None,
            StatusCode::Code191 => None,
            StatusCode::Code192 => None,
            StatusCode::Code193 => None,
            StatusCode::Code194 => None,
            StatusCode::Code195 => None,
            StatusCode::Code196 => None,
            StatusCode::Code197 => None,
            StatusCode::Code198 => None,
            StatusCode::Code199 => None,

            StatusCode::Ok => Some("OK"),
            StatusCode::Created => Some("Created"),
            StatusCode::Accepted => Some("Accepted"),
            StatusCode::NonAuthoritativeInformation => Some("Non-Authoritative Information"),
            StatusCode::NoContent => Some("No Content"),
            StatusCode::ResetContent => Some("Reset Content"),
            StatusCode::PartialContent => Some("Partial Content"),
            StatusCode::MultiStatus => Some("Multi-Status"),
            StatusCode::AlreadyReported => Some("Already Reported"),
            StatusCode::Code209 => None,
            StatusCode::Code210 => None,
            StatusCode::Code211 => None,
            StatusCode::Code212 => None,
            StatusCode::Code213 => None,
            StatusCode::Code214 => None,
            StatusCode::Code215 => None,
            StatusCode::Code216 => None,
            StatusCode::Code217 => None,
            StatusCode::Code218 => None,
            StatusCode::Code219 => None,
            StatusCode::Code220 => None,
            StatusCode::Code221 => None,
            StatusCode::Code222 => None,
            StatusCode::Code223 => None,
            StatusCode::Code224 => None,
            StatusCode::Code225 => None,
            StatusCode::ImUsed => Some("IM Used"),
            StatusCode::Code227 => None,
            StatusCode::Code228 => None,
            StatusCode::Code229 => None,
            StatusCode::Code230 => None,
            StatusCode::Code231 => None,
            StatusCode::Code232 => None,
            StatusCode::Code233 => None,
            StatusCode::Code234 => None,
            StatusCode::Code235 => None,
            StatusCode::Code236 => None,
            StatusCode::Code237 => None,
            StatusCode::Code238 => None,
            StatusCode::Code239 => None,
            StatusCode::Code240 => None,
            StatusCode::Code241 => None,
            StatusCode::Code242 => None,
            StatusCode::Code243 => None,
            StatusCode::Code244 => None,
            StatusCode::Code245 => None,
            StatusCode::Code246 => None,
            StatusCode::Code247 => None,
            StatusCode::Code248 => None,
            StatusCode::Code249 => None,
            StatusCode::Code250 => None,
            StatusCode::Code251 => None,
            StatusCode::Code252 => None,
            StatusCode::Code253 => None,
            StatusCode::Code254 => None,
            StatusCode::Code255 => None,
            StatusCode::Code256 => None,
            StatusCode::Code257 => None,
            StatusCode::Code258 => None,
            StatusCode::Code259 => None,
            StatusCode::Code260 => None,
            StatusCode::Code261 => None,
            StatusCode::Code262 => None,
            StatusCode::Code263 => None,
            StatusCode::Code264 => None,
            StatusCode::Code265 => None,
            StatusCode::Code266 => None,
            StatusCode::Code267 => None,
            StatusCode::Code268 => None,
            StatusCode::Code269 => None,
            StatusCode::Code270 => None,
            StatusCode::Code271 => None,
            StatusCode::Code272 => None,
            StatusCode::Code273 => None,
            StatusCode::Code274 => None,
            StatusCode::Code275 => None,
            StatusCode::Code276 => None,
            StatusCode::Code277 => None,
            StatusCode::Code278 => None,
            StatusCode::Code279 => None,
            StatusCode::Code280 => None,
            StatusCode::Code281 => None,
            StatusCode::Code282 => None,
            StatusCode::Code283 => None,
            StatusCode::Code284 => None,
            StatusCode::Code285 => None,
            StatusCode::Code286 => None,
            StatusCode::Code287 => None,
            StatusCode::Code288 => None,
            StatusCode::Code289 => None,
            StatusCode::Code290 => None,
            StatusCode::Code291 => None,
            StatusCode::Code292 => None,
            StatusCode::Code293 => None,
            StatusCode::Code294 => None,
            StatusCode::Code295 => None,
            StatusCode::Code296 => None,
            StatusCode::Code297 => None,
            StatusCode::Code298 => None,
            StatusCode::Code299 => None,

            StatusCode::MultipleChoices => Some("Multiple Choices"),
            StatusCode::MovedPermanently => Some("Moved Permanently"),
            StatusCode::Found => Some("Found"),
            StatusCode::SeeOther => Some("See Other"),
            StatusCode::NotModified => Some("Not Modified"),
            StatusCode::UseProxy => Some("Use Proxy"),
            StatusCode::SwitchProxy => Some("Switch Proxy"),
            StatusCode::TemporaryRedirect => Some("Temporary Redirect"),
            StatusCode::PermanentRedirect => Some("Permanent Redirect"),
            StatusCode::Code309 => None,
            StatusCode::Code310 => None,
            StatusCode::Code311 => None,
            StatusCode::Code312 => None,
            StatusCode::Code313 => None,
            StatusCode::Code314 => None,
            StatusCode::Code315 => None,
            StatusCode::Code316 => None,
            StatusCode::Code317 => None,
            StatusCode::Code318 => None,
            StatusCode::Code319 => None,
            StatusCode::Code320 => None,
            StatusCode::Code321 => None,
            StatusCode::Code322 => None,
            StatusCode::Code323 => None,
            StatusCode::Code324 => None,
            StatusCode::Code325 => None,
            StatusCode::Code326 => None,
            StatusCode::Code327 => None,
            StatusCode::Code328 => None,
            StatusCode::Code329 => None,
            StatusCode::Code330 => None,
            StatusCode::Code331 => None,
            StatusCode::Code332 => None,
            StatusCode::Code333 => None,
            StatusCode::Code334 => None,
            StatusCode::Code335 => None,
            StatusCode::Code336 => None,
            StatusCode::Code337 => None,
            StatusCode::Code338 => None,
            StatusCode::Code339 => None,
            StatusCode::Code340 => None,
            StatusCode::Code341 => None,
            StatusCode::Code342 => None,
            StatusCode::Code343 => None,
            StatusCode::Code344 => None,
            StatusCode::Code345 => None,
            StatusCode::Code346 => None,
            StatusCode::Code347 => None,
            StatusCode::Code348 => None,
            StatusCode::Code349 => None,
            StatusCode::Code350 => None,
            StatusCode::Code351 => None,
            StatusCode::Code352 => None,
            StatusCode::Code353 => None,
            StatusCode::Code354 => None,
            StatusCode::Code355 => None,
            StatusCode::Code356 => None,
            StatusCode::Code357 => None,
            StatusCode::Code358 => None,
            StatusCode::Code359 => None,
            StatusCode::Code360 => None,
            StatusCode::Code361 => None,
            StatusCode::Code362 => None,
            StatusCode::Code363 => None,
            StatusCode::Code364 => None,
            StatusCode::Code365 => None,
            StatusCode::Code366 => None,
            StatusCode::Code367 => None,
            StatusCode::Code368 => None,
            StatusCode::Code369 => None,
            StatusCode::Code370 => None,
            StatusCode::Code371 => None,
            StatusCode::Code372 => None,
            StatusCode::Code373 => None,
            StatusCode::Code374 => None,
            StatusCode::Code375 => None,
            StatusCode::Code376 => None,
            StatusCode::Code377 => None,
            StatusCode::Code378 => None,
            StatusCode::Code379 => None,
            StatusCode::Code380 => None,
            StatusCode::Code381 => None,
            StatusCode::Code382 => None,
            StatusCode::Code383 => None,
            StatusCode::Code384 => None,
            StatusCode::Code385 => None,
            StatusCode::Code386 => None,
            StatusCode::Code387 => None,
            StatusCode::Code388 => None,
            StatusCode::Code389 => None,
            StatusCode::Code390 => None,
            StatusCode::Code391 => None,
            StatusCode::Code392 => None,
            StatusCode::Code393 => None,
            StatusCode::Code394 => None,
            StatusCode::Code395 => None,
            StatusCode::Code396 => None,
            StatusCode::Code397 => None,
            StatusCode::Code398 => None,
            StatusCode::Code399 => None,

            StatusCode::BadRequest => Some("Bad Request"),
            StatusCode::Unauthorized => Some("Unauthorized"),
            StatusCode::PaymentRequired => Some("Payment Required"),
            StatusCode::Forbidden => Some("Forbidden"),
            StatusCode::NotFound => Some("Not Found"),
            StatusCode::MethodNotAllowed => Some("Method Not Allowed"),
            StatusCode::NotAcceptable => Some("Not Acceptable"),
            StatusCode::ProxyAuthenticationRequired => Some("Proxy Authentication Required"),
            StatusCode::RequestTimeout => Some("Request Timeout"),
            StatusCode::Conflict => Some("Conflict"),
            StatusCode::Gone => Some("Gone"),
            StatusCode::LengthRequired => Some("Length Required"),
            StatusCode::PreconditionFailed => Some("Precondition Failed"),
            StatusCode::RequestEntityTooLarge => Some("Request Entity Too Large"),
            StatusCode::RequestUriTooLong => Some("Request-URI Too Long"),
            StatusCode::UnsupportedMediaType => Some("Unsupported Media Type"),
            StatusCode::RequestedRangeNotSatisfiable => Some("Requested Range Not Satisfiable"),
            StatusCode::ExpectationFailed => Some("Expectation Failed"),
            StatusCode::ImATeapot => Some("I'm a teapot"),
            StatusCode::AuthenticationTimeout => Some("Authentication Timeout"),
            StatusCode::Code420 => None,
            StatusCode::Code421 => None,
            StatusCode::UnprocessableEntity => Some("Unprocessable Entity"),
            StatusCode::Locked => Some("Locked"),
            StatusCode::FailedDependency => Some("Failed Dependency"),
            StatusCode::UnorderedCollection => Some("Unordered Collection"),
            StatusCode::UpgradeRequired => Some("Upgrade Required"),
            StatusCode::Code427 => None,
            StatusCode::PreconditionRequired => Some("Precondition Required"),
            StatusCode::TooManyRequests => Some("Too Many Requests"),
            StatusCode::Code430 => None,
            StatusCode::RequestHeaderFieldsTooLarge => Some("Request Header Fields Too Large"),
            StatusCode::Code432 => None,
            StatusCode::Code433 => None,
            StatusCode::Code434 => None,
            StatusCode::Code435 => None,
            StatusCode::Code436 => None,
            StatusCode::Code437 => None,
            StatusCode::Code438 => None,
            StatusCode::Code439 => None,
            StatusCode::Code440 => None,
            StatusCode::Code441 => None,
            StatusCode::Code442 => None,
            StatusCode::Code443 => None,
            StatusCode::Code444 => None,
            StatusCode::Code445 => None,
            StatusCode::Code446 => None,
            StatusCode::Code447 => None,
            StatusCode::Code448 => None,
            StatusCode::Code449 => None,
            StatusCode::Code450 => None,
            StatusCode::UnavailableForLegalReasons => Some("Unavailable For Legal Reasons"),
            StatusCode::Code452 => None,
            StatusCode::Code453 => None,
            StatusCode::Code454 => None,
            StatusCode::Code455 => None,
            StatusCode::Code456 => None,
            StatusCode::Code457 => None,
            StatusCode::Code458 => None,
            StatusCode::Code459 => None,
            StatusCode::Code460 => None,
            StatusCode::Code461 => None,
            StatusCode::Code462 => None,
            StatusCode::Code463 => None,
            StatusCode::Code464 => None,
            StatusCode::Code465 => None,
            StatusCode::Code466 => None,
            StatusCode::Code467 => None,
            StatusCode::Code468 => None,
            StatusCode::Code469 => None,
            StatusCode::Code470 => None,
            StatusCode::Code471 => None,
            StatusCode::Code472 => None,
            StatusCode::Code473 => None,
            StatusCode::Code474 => None,
            StatusCode::Code475 => None,
            StatusCode::Code476 => None,
            StatusCode::Code477 => None,
            StatusCode::Code478 => None,
            StatusCode::Code479 => None,
            StatusCode::Code480 => None,
            StatusCode::Code481 => None,
            StatusCode::Code482 => None,
            StatusCode::Code483 => None,
            StatusCode::Code484 => None,
            StatusCode::Code485 => None,
            StatusCode::Code486 => None,
            StatusCode::Code487 => None,
            StatusCode::Code488 => None,
            StatusCode::Code489 => None,
            StatusCode::Code490 => None,
            StatusCode::Code491 => None,
            StatusCode::Code492 => None,
            StatusCode::Code493 => None,
            StatusCode::Code494 => None,
            StatusCode::Code495 => None,
            StatusCode::Code496 => None,
            StatusCode::Code497 => None,
            StatusCode::Code498 => None,
            StatusCode::Code499 => None,

            StatusCode::InternalServerError => Some("Internal Server Error"),
            StatusCode::NotImplemented => Some("Not Implemented"),
            StatusCode::BadGateway => Some("Bad Gateway"),
            StatusCode::ServiceUnavailable => Some("Service Unavailable"),
            StatusCode::GatewayTimeout => Some("Gateway Timeout"),
            StatusCode::HttpVersionNotSupported => Some("HTTP Version Not Supported"),
            StatusCode::VariantAlsoNegotiates => Some("Variant Also Negotiates"),
            StatusCode::InsufficientStorage => Some("Insufficient Storage"),
            StatusCode::LoopDetected => Some("Loop Detected"),
            StatusCode::Code509 => None,
            StatusCode::NotExtended => Some("Not Extended"),
            StatusCode::NetworkAuthenticationRequired => Some("Network Authentication Required"),
            StatusCode::Code512 => None,
            StatusCode::Code513 => None,
            StatusCode::Code514 => None,
            StatusCode::Code515 => None,
            StatusCode::Code516 => None,
            StatusCode::Code517 => None,
            StatusCode::Code518 => None,
            StatusCode::Code519 => None,
            StatusCode::Code520 => None,
            StatusCode::Code521 => None,
            StatusCode::Code522 => None,
            StatusCode::Code523 => None,
            StatusCode::Code524 => None,
            StatusCode::Code525 => None,
            StatusCode::Code526 => None,
            StatusCode::Code527 => None,
            StatusCode::Code528 => None,
            StatusCode::Code529 => None,
            StatusCode::Code530 => None,
            StatusCode::Code531 => None,
            StatusCode::Code532 => None,
            StatusCode::Code533 => None,
            StatusCode::Code534 => None,
            StatusCode::Code535 => None,
            StatusCode::Code536 => None,
            StatusCode::Code537 => None,
            StatusCode::Code538 => None,
            StatusCode::Code539 => None,
            StatusCode::Code540 => None,
            StatusCode::Code541 => None,
            StatusCode::Code542 => None,
            StatusCode::Code543 => None,
            StatusCode::Code544 => None,
            StatusCode::Code545 => None,
            StatusCode::Code546 => None,
            StatusCode::Code547 => None,
            StatusCode::Code548 => None,
            StatusCode::Code549 => None,
            StatusCode::Code550 => None,
            StatusCode::Code551 => None,
            StatusCode::Code552 => None,
            StatusCode::Code553 => None,
            StatusCode::Code554 => None,
            StatusCode::Code555 => None,
            StatusCode::Code556 => None,
            StatusCode::Code557 => None,
            StatusCode::Code558 => None,
            StatusCode::Code559 => None,
            StatusCode::Code560 => None,
            StatusCode::Code561 => None,
            StatusCode::Code562 => None,
            StatusCode::Code563 => None,
            StatusCode::Code564 => None,
            StatusCode::Code565 => None,
            StatusCode::Code566 => None,
            StatusCode::Code567 => None,
            StatusCode::Code568 => None,
            StatusCode::Code569 => None,
            StatusCode::Code570 => None,
            StatusCode::Code571 => None,
            StatusCode::Code572 => None,
            StatusCode::Code573 => None,
            StatusCode::Code574 => None,
            StatusCode::Code575 => None,
            StatusCode::Code576 => None,
            StatusCode::Code577 => None,
            StatusCode::Code578 => None,
            StatusCode::Code579 => None,
            StatusCode::Code580 => None,
            StatusCode::Code581 => None,
            StatusCode::Code582 => None,
            StatusCode::Code583 => None,
            StatusCode::Code584 => None,
            StatusCode::Code585 => None,
            StatusCode::Code586 => None,
            StatusCode::Code587 => None,
            StatusCode::Code588 => None,
            StatusCode::Code589 => None,
            StatusCode::Code590 => None,
            StatusCode::Code591 => None,
            StatusCode::Code592 => None,
            StatusCode::Code593 => None,
            StatusCode::Code594 => None,
            StatusCode::Code595 => None,
            StatusCode::Code596 => None,
            StatusCode::Code597 => None,
            StatusCode::Code598 => None,
            StatusCode::Code599 => None,
        }
    }

    /// Determine the class of a status code, based on its first digit.
    #[unstable]
    pub fn class(&self) -> StatusClass {
        let code = *self as u16;  // Range of possible values: 100..599.
        // We could match 100..199 &c., but this way we avoid unreachable!() at the end.
        if code < 200 {
            StatusClass::Informational
        } else if code < 300 {
            StatusClass::Success
        } else if code < 400 {
            StatusClass::Redirection
        } else if code < 500 {
            StatusClass::ClientError
        } else {
            StatusClass::ServerError
        }
    }

    /// Check if class is Informational.
    #[unstable]
    pub fn is_informational(&self) -> bool {
        self.class() == StatusClass::Informational
    }

    /// Check if class is Success.
    #[unstable]
    pub fn is_success(&self) -> bool {
        self.class() == StatusClass::Success
    }

    /// Check if class is Redirection
    #[unstable]
    pub fn is_redirection(&self) -> bool {
        self.class() == StatusClass::Redirection
    }

    /// Check if class is ClientError.
    #[unstable]
    pub fn is_client_error(&self) -> bool {
        self.class() == StatusClass::ClientError
    }

    /// Check if class is ServerError.
    #[unstable]
    pub fn is_server_error(&self) -> bool {
        self.class() == StatusClass::ServerError
    }
}

/// Formats the status code, *including* the canonical reason.
///
/// ```rust
/// # use hyper::status::StatusCode::{ImATeapot, Code123};
/// assert_eq!(format!("{}", ImATeapot).as_slice(),
///            "418 I'm a teapot");
/// assert_eq!(format!("{}", Code123).as_slice(),
///            "123 <unknown status code>");
/// ```
///
/// If you wish to just include the number, cast to a u16 instead.
impl fmt::Show for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", *self as u16,
               self.canonical_reason().unwrap_or("<unknown status code>"))
    }
}

// Specified manually because the codegen for derived is slow (at the time of writing on the machine
// of writing, 1.2 seconds) and verbose (though the optimiser cuts it down to size).
impl PartialEq for StatusCode {
    #[inline]
    fn eq(&self, other: &StatusCode) -> bool {
        *self as u16 == *other as u16
    }
}

impl Eq for StatusCode {}

// Ditto (though #[deriving(Clone)] only takes about 0.4 seconds).
impl Clone for StatusCode {
    #[inline]
    fn clone(&self) -> StatusCode {
        *self
    }
}

// Of the other common derivable traits, I didn’t measure them, but I guess they would be slow too.

impl FromPrimitive for StatusCode {
    fn from_i64(n: i64) -> Option<StatusCode> {
        if n < 100 || n > 599 {
            None
        } else {
            Some(unsafe { transmute::<u16, StatusCode>(n as u16) })
        }
    }

    fn from_u64(n: u64) -> Option<StatusCode> {
        if n < 100 || n > 599 {
            None
        } else {
            Some(unsafe { transmute::<u16, StatusCode>(n as u16) })
        }
    }
}

impl PartialOrd for StatusCode {
    #[inline]
    fn partial_cmp(&self, other: &StatusCode) -> Option<Ordering> {
        (*self as u16).partial_cmp(&(*other as u16))
    }
}

impl Ord for StatusCode {
    #[inline]
    fn cmp(&self, other: &StatusCode) -> Ordering {
        if *self < *other {
            Less
        } else if *self > *other {
            Greater
        } else {
            Equal
        }
    }
}

impl ToPrimitive for StatusCode {
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }
}

/// The class of an HTTP `Status-Code`.
///
/// [RFC 2616, section 6.1.1 (Status Code and Reason
/// Phrase)](https://tools.ietf.org/html/rfc2616#section-6.1.1):
///
/// > The first digit of the Status-Code defines the class of response. The
/// > last two digits do not have any categorization role.
/// >
/// > ...
/// >
/// > HTTP status codes are extensible. HTTP applications are not required
/// > to understand the meaning of all registered status codes, though such
/// > understanding is obviously desirable. However, applications MUST
/// > understand the class of any status code, as indicated by the first
/// > digit, and treat any unrecognized response as being equivalent to the
/// > x00 status code of that class, with the exception that an
/// > unrecognized response MUST NOT be cached. For example, if an
/// > unrecognized status code of 431 is received by the client, it can
/// > safely assume that there was something wrong with its request and
/// > treat the response as if it had received a 400 status code. In such
/// > cases, user agents SHOULD present to the user the entity returned
/// > with the response, since that entity is likely to include human-
/// > readable information which will explain the unusual status.
///
/// This can be used in cases where a status code’s meaning is unknown, also,
/// to get the appropriate *category* of status.
///
/// For HTTP/2.0, the 1xx Informational class is invalid.
#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[unstable]
pub enum StatusClass {
    /// 1xx: Informational - Request received, continuing process
    #[unstable]
    Informational = 100,

    /// 2xx: Success - The action was successfully received, understood, and accepted
    #[unstable]
    Success = 200,

    /// 3xx: Redirection - Further action must be taken in order to complete the request
    #[unstable]
    Redirection = 300,

    /// 4xx: Client Error - The request contains bad syntax or cannot be fulfilled
    #[unstable]
    ClientError = 400,

    /// 5xx: Server Error - The server failed to fulfill an apparently valid request
    #[unstable]
    ServerError = 500,
}

impl StatusClass {
    /// Get the default status code for the class.
    ///
    /// This produces the x00 status code; thus, for `ClientError` (4xx), for example, this will
    /// produce `BadRequest` (400):
    ///
    /// ```rust
    /// # use hyper::status::StatusClass::ClientError;
    /// # use hyper::status::StatusCode::BadRequest;
    /// assert_eq!(ClientError.default_code(), BadRequest);
    /// ```
    ///
    /// The use for this is outlined in [RFC 2616, section 6.1.1 (Status Code and Reason
    /// Phrase)](https://tools.ietf.org/html/rfc2616#section-6.1.1):
    ///
    /// > HTTP status codes are extensible. HTTP applications are not required
    /// > to understand the meaning of all registered status codes, though such
    /// > understanding is obviously desirable. However, applications MUST
    /// > understand the class of any status code, as indicated by the first
    /// > digit, and treat any unrecognized response as being equivalent to the
    /// > x00 status code of that class, with the exception that an
    /// > unrecognized response MUST NOT be cached. For example, if an
    /// > unrecognized status code of 431 is received by the client, it can
    /// > safely assume that there was something wrong with its request and
    /// > treat the response as if it had received a 400 status code. In such
    /// > cases, user agents SHOULD present to the user the entity returned
    /// > with the response, since that entity is likely to include human-
    /// > readable information which will explain the unusual status.
    ///
    /// This is demonstrated thusly (I’ll use 432 rather than 431 as 431 *is* now in use):
    ///
    /// ```rust
    /// # use hyper::status::StatusCode::{Code432, BadRequest};
    /// // Suppose we have received this status code.
    /// let status = Code432;
    ///
    /// // Uh oh! Don’t know what to do with it.
    /// // Let’s fall back to the default:
    /// let status = status.class().default_code();
    ///
    /// // And look! That is 400 Bad Request.
    /// assert_eq!(status, BadRequest);
    /// // So now let’s treat it as that.
    /// ```
    #[inline]
    #[unstable]
    pub fn default_code(&self) -> StatusCode {
        unsafe { transmute::<StatusClass, StatusCode>(*self) }
    }
}

impl ToPrimitive for StatusClass {
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }
}
