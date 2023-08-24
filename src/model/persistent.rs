use crate::model::*;
use serde_json::Value;
use std::time::Instant;

/*
final case class HttpStub(
    @BsonKey("_id")
    @description("id мока")
    id: SID[HttpStub],
    @description("Время создания мока")
    created: Instant,
    @description("Тип конфигурации")
    scope: Scope,
    @description("Количество возможных срабатываний. Имеет смысл только для scope=countdown")
    times: Option[Int] = Some(1),
    serviceSuffix: String,
    @description("Название мока")
    name: String,
    @description("HTTP метод")
    method: HttpMethod,
    @description("Суффикс пути, по которому срабатывает мок")
    path: Option[String],
    pathPattern: Option[Regex],
    seed: Option[Json],
    @description("Предикат для поиска состояния")
    state: Option[Map[JsonOptic, Map[Keyword.Json, Json]]],
    @description("Спецификация запроса")
    request: HttpStubRequest,
    @description("Данные, записываемые в базу")
    persist: Option[Map[JsonOptic, Json]],
    @description("Спецификация ответа")
    response: HttpStubResponse,
    @description("Спецификация колбека")
    callback: Option[Callback],
    @description("Тэги")
    labels: Seq[String] = Seq.empty
)
 */

pub struct HttpStub {
    id: String,
    created: Instant,
    scope: Scope,
    times: Option<u64>,
    service_suffix: String,
    name: String,
    method: HttpMethod,
    path: Option<String>,
    path_pattern: Option<String>,
    seed: Option<Value>
}