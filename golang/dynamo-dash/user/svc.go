package user

import (
	"dynamo-dash/util"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"strings"
)

type Service interface {
	Create(request CreateUserRequest) string
	Login(username string, password string) bool
	GetByUsername(username string) *User
	GetByUUID(uuid string) *User
}

var _ Service = &service{}

type service struct {
	dynamodbClient *dynamodb.DynamoDB
}

func NewService(dynamodbClient *dynamodb.DynamoDB) Service {
	return service{
		dynamodbClient: dynamodbClient,
	}
}

func (s service) GetByUsername(username string) *User {
	panic("implement me")
}

func (s service) GetByUUID(uuid string) *User {
	key := struct {
		PK string
	}{
		PK: util.PartitionKey(Prefix, uuid),
	}

	input := &dynamodb.GetItemInput{
		Key:       util.MustMarshalMap(key),
		TableName: aws.String(util.TableName),
	}

	result, err := s.dynamodbClient.GetItem(input)
	if err != nil {
		return nil
	}
	if result.Item == nil {
		return nil
	}
	user := User{}
	util.MustUnMarshalMap(result.Item, &user)
	strings.ReplaceAll(Prefix+"#", user.Id, "") // remove the prefix
	return &user
}

// Create creates a new users if one doesn't exist.
// returns user's uuid if it is created successfully otherwise return empty string.
func (s service) Create(request CreateUserRequest) string {
	panic("implement me")
}

func (s service) Login(username string, password string) bool {
	panic("implement me")
}
