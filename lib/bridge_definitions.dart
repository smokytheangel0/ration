// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.48.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names

import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

abstract class Native {
  Future<Platform> platform({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kPlatformConstMeta;

  Future<bool> rustReleaseMode({dynamic hint});

  FlutterRustBridgeTaskConstMeta get kRustReleaseModeConstMeta;

  Future<ItemInfo> getNutrition({required Item item, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kGetNutritionConstMeta;

  Future<List<Item>> searchLocalItems({required String input, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSearchLocalItemsConstMeta;
}

enum Item {
  LargeChickenEgg,
  LargeChickenEggYolk,
  LargeChickenEggWhite,
  TableSalt,
  TableSugar,
  Water,
  WheatFlour,
  ActiveDryYeast,
  CowButter,
  NoMatch,
}

class ItemInfo {
  final Nutrition? nutrition;
  final String fileName;
  final String displayName;

  ItemInfo({
    this.nutrition,
    required this.fileName,
    required this.displayName,
  });
}

class Nutrition {
  final Item variant;
  final double? unitWeight;
  final double literWeight;
  final double calories;
  final double caloriesFromFat;
  final double totalFatWeight;
  final double saturatedFatWeight;
  final double polyunsaturatedFatWeight;
  final double monounsaturatedFatWeight;
  final double cholesterolWeight;
  final double sodiumWeight;
  final double potassiumWeight;
  final double carbohydrateWeight;
  final double fiberWeight;
  final double sugarWeight;
  final double proteinWeight;

  Nutrition({
    required this.variant,
    this.unitWeight,
    required this.literWeight,
    required this.calories,
    required this.caloriesFromFat,
    required this.totalFatWeight,
    required this.saturatedFatWeight,
    required this.polyunsaturatedFatWeight,
    required this.monounsaturatedFatWeight,
    required this.cholesterolWeight,
    required this.sodiumWeight,
    required this.potassiumWeight,
    required this.carbohydrateWeight,
    required this.fiberWeight,
    required this.sugarWeight,
    required this.proteinWeight,
  });
}

enum Platform {
  Unknown,
  Android,
  Ios,
  Windows,
  Unix,
  MacIntel,
  MacApple,
  Wasm,
}