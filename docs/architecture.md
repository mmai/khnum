# Architecture

## Users

## Registration

* Inspiration: https://neosmart.net/blog/2015/using-hmac-signatures-to-avoid-database-writes/
* Hashing is done via bcrypt

Process : 

Form (email) -> post to
/register/request 
  -> check email not taken
  -> send confirmation link 

/register/{hashlink}/{email}/{expires_at} 
  -> check link valid 
  -> init session with email

Form (username, password) (with session cookie) -> post to
/register/validate
  -> get session email
  -> check email not taken
  -> hash password
  -> create user

## Forgotten  password TODO

Form (email) -> post to
/user/forgotten 
  -> check email exists
  -> send temporary link 

/user/forgotten/{hashlink}
  -> check link valid 
  -> init session

Form (password) (with session cookie) -> post to
/user/changepassword
  -> get session user
  -> hash password
  -> update user

## Login TODO

Form (login, password) -> post to
/login
  -> check user exist for login + password
  -> init session

protected pages :
  -> get session cookie
  -> check user exists
  -> check user rights

## Logout TODO

/logout
 -> invalidate session cookie
