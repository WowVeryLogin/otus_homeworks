// Вы являетесь разработчиком системы управления транспортным парком. Ваша задача — создать программу, которая моделирует парк транспортных средств
// Требования:
//
// В вашем парке могут находиться:
// * велосипеды (характеристики: горный или городской)
// * автомобили (характеристики: кол-во дверей - u8 и тип топлива: бензин или электричество)
// * автобус (характеристики: вместимость пассажиров - u16)
//
// Каждое транспортное средство имеет следующие параметры:
// * тип транспортного средства (велосипед/автомобиль/автобус)
// * регистрационный номер - String
// * пробег в км - f64
//
// Каждое транспортное средство должно иметь функцию печати в формате:
// Регистрационный номер: <номер>, Тип: <тип транспорта>, Дополнительно: <дополнительная информация>.
// Пример:
// Регистрационный номер: A123BC, Тип: Автомобиль, 4 двери, бензин.
// Регистрационный номер: B456CD, Тип: Автобус, вместимость: 50 человек.
//
// Создайте массив с как минимум с 5 различными транспортными средствами
// и напишите функцию stats_with_filter которая принимает этот массив, и считает только транспортные средства
// соответствующие критерию фильтрации и возвращает кортеж с 4 параметрами
// * общее кол-во транспортных средств
// * кол-во автомобилей
// * кол-во автобусов
// * кол-во велосипедов
//
// Критерии фильтрации:
// * по пробегу (пробег меньше чем заданный)
// * по пробегу (пробег больше чем заданный)

fn stats_with_filter(
    vehicles: &[Vehicle],
    filter: Filter,
) -> (u64, u64, u64, u64) {
    // ваш код
}

fn main() {
    let vehicles = [];
    let stats = stats_by_type(&vehicles, Filter::MillageMoreThan(10000.0));
    // ваш код для печати статистики
}