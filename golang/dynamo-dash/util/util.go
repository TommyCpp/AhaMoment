package util

import (
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
	"github.com/google/uuid"
)

const TableName string = "Delivery"

func PartitionKey(prefix string, key string) string {
	return prefix + "#" + key
}

func MustUUID() string {
	uid, _ := uuid.NewUUID()
	return uid.String()
}

func MustMarshalMap(any interface{}) map[string]*dynamodb.AttributeValue {
	result, _ := dynamodbattribute.MarshalMap(any)
	return result
}

func MustUnMarshalMap(m map[string]*dynamodb.AttributeValue, out interface{}) {
	dynamodbattribute.UnmarshalMap(m, out)
}
