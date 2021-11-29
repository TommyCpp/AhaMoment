package user

const Prefix string = "USER"

//User user model
type User struct {
	CreateUserRequest
	Id string `json:"id" dynamodb: "partitionKey"`
}

type CreateUserRequest struct {
	Name string `json:"name"`
	Pass string `json:"pass"`
}
