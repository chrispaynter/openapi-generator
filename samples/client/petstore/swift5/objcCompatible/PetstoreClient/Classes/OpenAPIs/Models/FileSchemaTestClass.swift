//
// FileSchemaTestClass.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation



@objc public class FileSchemaTestClass: NSObject, Codable { 

    public var file: File?
    public var files: [File]?

    public init(file: File?, files: [File]?) {
        self.file = file
        self.files = files
    }

}