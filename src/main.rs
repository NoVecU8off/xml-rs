use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

// Представляет собой значение поля FIX
#[derive(Debug, Clone)]
struct FieldValue {
    description: String,
    enum_value: String,
}

// Представляет собой поле FIX
#[derive(Debug, Clone)]
struct Field {
    name: String,
    number: String,
    field_type: String,
    values: HashMap<String, String>, // description -> enum_value
}

// Представляет собой группу полей FIX
#[derive(Debug, Clone)]
struct Group {
    name: String,
    number: String,
    delim: String,
    order: Vec<String>,
    fields: Vec<String>,
    components: Vec<String>,
    sub_groups: Vec<Group>,
}

// Представляет собой компонент FIX
#[derive(Debug, Clone)]
struct Component {
    name: String,
    fields: Vec<String>,
    components: Vec<String>,
    groups: Vec<Group>,
}

// Представляет собой сообщение FIX
#[derive(Debug, Clone)]
struct Message {
    name: String,
    msgtype: String,
    required_fields: Vec<String>,
    fields: Vec<String>,
    components: Vec<String>,
    groups: Vec<Group>,
}

// Основной класс для обработки XML спецификаций FIX
struct FixProcessor {
    major_version: String,
    minor_version: String,
    header_fields: Vec<String>,
    trailer_fields: Vec<String>,
    fields: HashMap<String, Field>,
    components: HashMap<String, Component>,
    messages: Vec<Message>,
    msg_to_type: HashMap<String, String>,
    output_dir: PathBuf,
    namespace: String,
    begin_string: String,
}

impl FixProcessor {
    fn new(filename: &str, output_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let xml_content = fs::read_to_string(filename)?;
        let mut reader = Reader::from_str(&xml_content);
        reader.config_mut().trim_text(true);

        // Извлекаем версию
        let mut major = String::new();
        let mut minor = String::new();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"fix" => {
                    for attr in e.attributes() {
                        let attr = attr?;
                        let name = attr.key.as_ref();
                        let value = attr.value.as_ref();
                        if name == b"major" {
                            major = String::from_utf8_lossy(value).to_string();
                        } else if name == b"minor" {
                            minor = String::from_utf8_lossy(value).to_string();
                        }
                    }
                    break;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
        }

        // Вычисляем namespace и begin_string
        let file_basename = Path::new(filename)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        let type_part = if file_basename.starts_with("FIXT") {
            "FIXT"
        } else {
            "FIX"
        };

        let namespace = format!("{}{}{}", type_part, major, minor);
        let begin_string = if type_part == "FIX" && major.parse::<i32>().unwrap_or(0) >= 5 {
            "FIXT.1.1".to_string()
        } else {
            format!("{}.{}.{}", type_part, major, minor)
        };

        Ok(FixProcessor {
            major_version: major,
            minor_version: minor,
            header_fields: Vec::new(),
            trailer_fields: Vec::new(),
            fields: HashMap::new(),
            components: HashMap::new(),
            messages: Vec::new(),
            msg_to_type: HashMap::new(),
            output_dir: output_dir.to_path_buf(),
            namespace,
            begin_string,
        })
    }

    fn process(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Processing {}", filename);

        let xml_content = fs::read_to_string(filename)?;

        // Parse fields
        self.parse_fields(&xml_content)?;

        // Parse components
        self.parse_components(&xml_content)?;

        // Parse header
        self.parse_header(&xml_content)?;

        // Parse trailer
        self.parse_trailer(&xml_content)?;

        // Parse messages
        self.parse_messages(&xml_content)?;

        // Generate Rust code
        self.generate_rust_code()?;

        Ok(())
    }

    fn parse_fields(&mut self, xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);

        let mut inside_fields = false;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"fields" => {
                    inside_fields = true;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"fields" => {
                    inside_fields = false;
                }
                Ok(Event::Start(ref e)) if inside_fields && e.name().as_ref() == b"field" => {
                    let mut name = String::new();
                    let mut number = String::new();
                    let mut field_type = String::new();

                    for attr in e.attributes() {
                        let attr = attr?;
                        let attr_name = attr.key.as_ref();
                        let attr_value = String::from_utf8_lossy(&attr.value).to_string();

                        if attr_name == b"name" {
                            name = attr_value;
                        } else if attr_name == b"number" {
                            number = attr_value;
                        } else if attr_name == b"type" {
                            field_type = attr_value;
                        }
                    }

                    let mut values = HashMap::new();

                    // Parse values
                    let mut value_buf = Vec::new();
                    loop {
                        match reader.read_event_into(&mut value_buf) {
                            Ok(Event::Start(ref e)) if e.name().as_ref() == b"value" => {
                                let mut enum_value = String::new();
                                let mut description = String::new();

                                for attr in e.attributes() {
                                    let attr = attr?;
                                    let attr_name = attr.key.as_ref();
                                    let attr_value =
                                        String::from_utf8_lossy(&attr.value).to_string();

                                    if attr_name == b"enum" {
                                        enum_value = attr_value;
                                    } else if attr_name == b"description" {
                                        description = attr_value;
                                    }
                                }

                                values.insert(description, enum_value);
                            }
                            Ok(Event::End(ref e)) if e.name().as_ref() == b"field" => {
                                break;
                            }
                            Ok(Event::Eof) => break,
                            Err(e) => return Err(e.into()),
                            _ => (),
                        }
                    }

                    self.fields.insert(
                        name.clone(),
                        Field {
                            name,
                            number,
                            field_type,
                            values,
                        },
                    );
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_components(&mut self, xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);

        let mut inside_components = false;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"components" => {
                    inside_components = true;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"components" => {
                    inside_components = false;
                }
                Ok(Event::Start(ref e))
                    if inside_components && e.name().as_ref() == b"component" =>
                {
                    let mut name = String::new();

                    for attr in e.attributes() {
                        let attr = attr?;
                        let attr_name = attr.key.as_ref();
                        let attr_value = String::from_utf8_lossy(&attr.value).to_string();

                        if attr_name == b"name" {
                            name = attr_value;
                        }
                    }

                    let component = self.parse_component_content(&mut reader, &name)?;
                    self.components.insert(name, component);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_component_content(
        &self,
        reader: &mut Reader<&[u8]>,
        name: &str,
    ) -> Result<Component, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        let mut components = Vec::new();
        let mut groups = Vec::new();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"field" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"name" {
                                let field_name = String::from_utf8_lossy(&attr.value).to_string();
                                fields.push(field_name);
                            }
                        }
                    }
                    b"component" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"name" {
                                let component_name =
                                    String::from_utf8_lossy(&attr.value).to_string();
                                components.push(component_name);
                            }
                        }
                    }
                    b"group" => {
                        let group = self.parse_group(reader, e)?;
                        groups.push(group);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) if e.name().as_ref() == b"component" => {
                    break;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(Component {
            name: name.to_string(),
            fields,
            components,
            groups,
        })
    }

    fn parse_group(
        &self,
        reader: &mut Reader<&[u8]>,
        start_tag: &BytesStart,
    ) -> Result<Group, Box<dyn std::error::Error>> {
        let mut name = String::new();
        let mut number = String::new();

        for attr in start_tag.attributes() {
            let attr = attr?;
            let attr_name = attr.key.as_ref();
            let attr_value = String::from_utf8_lossy(&attr.value).to_string();

            if attr_name == b"name" {
                name = attr_value;
                // Находим номер поля по имени
                if let Some(field) = self.fields.get(&name) {
                    number = field.number.clone();
                }
            }
        }

        let mut fields = Vec::new();
        let mut components = Vec::new();
        let mut sub_groups = Vec::new();

        // Первое поле будет разделителем
        let mut delim = String::new();
        let mut order = Vec::new();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"field" => {
                            for attr in e.attributes() {
                                let attr = attr?;
                                if attr.key.as_ref() == b"name" {
                                    let field_name =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                    fields.push(field_name.clone());

                                    if let Some(field) = self.fields.get(&field_name) {
                                        if delim.is_empty() {
                                            delim = field.number.clone();
                                        }
                                        order.push(field.number.clone());
                                    }
                                }
                            }
                        }
                        b"component" => {
                            for attr in e.attributes() {
                                let attr = attr?;
                                if attr.key.as_ref() == b"name" {
                                    let component_name =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                    components.push(component_name.clone());

                                    // Добавляем поля компонента в order
                                    if let Some(component) = self.components.get(&component_name) {
                                        for field_name in &component.fields {
                                            if let Some(field) = self.fields.get(field_name) {
                                                if delim.is_empty() {
                                                    delim = field.number.clone();
                                                }
                                                order.push(field.number.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        b"group" => {
                            let group = self.parse_group(reader, e)?;
                            sub_groups.push(group);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"group" => {
                    break;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(Group {
            name,
            number,
            delim,
            order,
            fields,
            components,
            sub_groups,
        })
    }

    fn parse_header(&mut self, xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);

        let mut inside_header = false;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"header" => {
                    inside_header = true;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"header" => {
                    inside_header = false;
                }
                Ok(Event::Start(ref e)) if inside_header && e.name().as_ref() == b"field" => {
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            let field_name = String::from_utf8_lossy(&attr.value).to_string();
                            self.header_fields.push(field_name);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_trailer(&mut self, xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);

        let mut inside_trailer = false;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"trailer" => {
                    inside_trailer = true;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"trailer" => {
                    inside_trailer = false;
                }
                Ok(Event::Start(ref e)) if inside_trailer && e.name().as_ref() == b"field" => {
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            let field_name = String::from_utf8_lossy(&attr.value).to_string();
                            self.trailer_fields.push(field_name);
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_messages(&mut self, xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml_content);
        reader.config_mut().trim_text(true);

        let mut inside_messages = false;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"messages" => {
                    inside_messages = true;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"messages" => {
                    inside_messages = false;
                }
                Ok(Event::Start(ref e)) if inside_messages && e.name().as_ref() == b"message" => {
                    let mut name = String::new();
                    let mut msgtype = String::new();

                    for attr in e.attributes() {
                        let attr = attr?;
                        let attr_name = attr.key.as_ref();
                        let attr_value = String::from_utf8_lossy(&attr.value).to_string();

                        if attr_name == b"name" {
                            name = attr_value;
                        } else if attr_name == b"msgtype" {
                            msgtype = attr_value;
                        }
                    }

                    // Запоминаем соответствие имени сообщения и его типа
                    self.msg_to_type.insert(name.clone(), msgtype.clone());

                    // Парсим содержимое сообщения
                    let message = self.parse_message_content(&mut reader, &name, &msgtype)?;
                    self.messages.push(message);
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(())
    }

    fn parse_message_content(
        &self,
        reader: &mut Reader<&[u8]>,
        name: &str,
        msgtype: &str,
    ) -> Result<Message, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        let mut required_fields = Vec::new();
        let mut components = Vec::new();
        let mut groups = Vec::new();

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"field" => {
                        let mut field_name = String::new();
                        let mut required = false;

                        for attr in e.attributes() {
                            let attr = attr?;
                            let attr_name = attr.key.as_ref();
                            let attr_value = String::from_utf8_lossy(&attr.value).to_string();

                            if attr_name == b"name" {
                                field_name = attr_value;
                            } else if attr_name == b"required" && attr_value == "Y" {
                                required = true;
                            }
                        }

                        fields.push(field_name.clone());
                        if required {
                            required_fields.push(field_name);
                        }
                    }
                    b"component" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"name" {
                                let component_name =
                                    String::from_utf8_lossy(&attr.value).to_string();
                                components.push(component_name);
                            }
                        }
                    }
                    b"group" => {
                        let group = self.parse_group(reader, e)?;
                        groups.push(group);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) if e.name().as_ref() == b"message" => {
                    break;
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(Message {
            name: name.to_string(),
            msgtype: msgtype.to_string(),
            required_fields,
            fields,
            components,
            groups,
        })
    }

    fn generate_rust_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Создаем директорию для вывода, если она не существует
        fs::create_dir_all(&self.output_dir)?;

        // Генерируем модули
        self.generate_mod_rs()?;

        // Генерируем структуры для полей
        self.generate_fields_rs()?;

        // Генерируем заголовки и трейлеры
        self.generate_header_trailer_rs()?;

        // Генерируем компоненты
        self.generate_components_rs()?;

        // Генерируем сообщения
        self.generate_messages_rs()?;

        // Генерируем enums для значений полей
        self.generate_field_enums_rs()?;

        Ok(())
    }

    fn generate_mod_rs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.output_dir.join("mod.rs");
        let mut file = File::create(file_path)?;

        writeln!(file, "//! {} protocol implementation", self.namespace)?;
        writeln!(file, "//!")?;
        writeln!(
            file,
            "//! Generated from FIX specification version {}.{}",
            self.major_version, self.minor_version
        )?;
        writeln!(file)?;

        writeln!(file, "pub mod fields;")?;
        writeln!(file, "pub mod header;")?;
        writeln!(file, "pub mod trailer;")?;
        writeln!(file, "pub mod components;")?;
        writeln!(file, "pub mod messages;")?;
        writeln!(file, "pub mod field_enums;")?;
        writeln!(file)?;

        writeln!(file, "pub use fields::*;")?;
        writeln!(file, "pub use header::*;")?;
        writeln!(file, "pub use trailer::*;")?;
        writeln!(file, "pub use components::*;")?;
        writeln!(file, "pub use messages::*;")?;
        writeln!(file, "pub use field_enums::*;")?;
        writeln!(file)?;

        writeln!(file, "/// FIX BeginString value for this version")?;
        writeln!(
            file,
            "pub const BEGIN_STRING: &str = \"{}\";",
            self.begin_string
        )?;

        Ok(())
    }

    fn generate_fields_rs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.output_dir.join("fields.rs");
        let mut file = File::create(file_path)?;

        writeln!(file, "//! Field definitions for {}", self.namespace)?;
        writeln!(file)?;
        writeln!(file, "use std::fmt;")?;
        writeln!(file, "use super::field_enums::*;")?;
        writeln!(file)?;

        // Генерируем перечисление для всех полей
        writeln!(file, "/// Enum of all field tags in {}", self.namespace)?;
        writeln!(file, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
        writeln!(file, "pub enum FieldTag {{")?;

        for field in self.fields.values() {
            writeln!(file, "    /// {} field", field.name)?;
            writeln!(file, "    {} = {},", field.name, field.number)?;
        }

        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация Display для FieldTag
        writeln!(file, "impl fmt::Display for FieldTag {{")?;
        writeln!(
            file,
            "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
        )?;
        writeln!(file, "        write!(f, \"{{}}\", *self as u32)")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация From<u32> для FieldTag
        writeln!(file, "impl TryFrom<u32> for FieldTag {{")?;
        writeln!(file, "    type Error = &'static str;")?;
        writeln!(
            file,
            "    fn try_from(value: u32) -> Result<Self, Self::Error> {{"
        )?;
        writeln!(file, "        match value {{")?;

        for field in self.fields.values() {
            writeln!(
                file,
                "            {} => Ok(FieldTag::{}),",
                field.number, field.name
            )?;
        }

        writeln!(file, "            _ => Err(\"Unknown field tag\")")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Генерируем перечисление для типов полей
        writeln!(file, "/// Field value types")?;
        writeln!(file, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
        writeln!(file, "pub enum FieldValue {{")?;
        writeln!(file, "    String(String),")?;
        writeln!(file, "    Char(char),")?;
        writeln!(file, "    Int(i64),")?;
        writeln!(file, "    Float(f64),")?;
        writeln!(file, "    Boolean(bool),")?;
        writeln!(file, "    Data(Vec<u8>),")?;
        writeln!(file, "    Date(String),")?;
        writeln!(file, "    Time(String),")?;
        writeln!(file, "    DateTime(String),")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация методов для FieldValue
        writeln!(file, "impl FieldValue {{")?;
        writeln!(file, "    /// Get field value as string")?;
        writeln!(file, "    pub fn as_string(&self) -> String {{")?;
        writeln!(file, "        match self {{")?;
        writeln!(file, "            FieldValue::String(s) => s.clone(),")?;
        writeln!(file, "            FieldValue::Char(c) => c.to_string(),")?;
        writeln!(file, "            FieldValue::Int(i) => i.to_string(),")?;
        writeln!(file, "            FieldValue::Float(f) => f.to_string(),")?;
        writeln!(file, "            FieldValue::Boolean(b) => if *b {{ \"Y\".to_string() }} else {{ \"N\".to_string() }},")?;
        writeln!(
            file,
            "            FieldValue::Data(d) => String::from_utf8_lossy(d).to_string(),"
        )?;
        writeln!(file, "            FieldValue::Date(d) => d.clone(),")?;
        writeln!(file, "            FieldValue::Time(t) => t.clone(),")?;
        writeln!(file, "            FieldValue::DateTime(dt) => dt.clone(),")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Генерируем структуру Field
        writeln!(file, "/// A single FIX field with tag and value")?;
        writeln!(file, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
        writeln!(file, "pub struct Field {{")?;
        writeln!(file, "    pub tag: FieldTag,")?;
        writeln!(file, "    pub value: FieldValue,")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация Field
        writeln!(file, "impl Field {{")?;
        writeln!(file, "    /// Create a new field with tag and value")?;
        writeln!(
            file,
            "    pub fn new(tag: FieldTag, value: FieldValue) -> Self {{"
        )?;
        writeln!(file, "        Field {{ tag, value }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "    ")?;
        writeln!(file, "    /// Format field as FIX string (tag=value\\x01)")?;
        writeln!(file, "    pub fn to_fix_string(&self) -> String {{")?;
        writeln!(
            file,
            "        format!(\"{{}={}}\\x01\", self.tag, self.value.as_string())"
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;

        Ok(())
    }

    fn generate_header_trailer_rs(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Генерируем header.rs
        let header_path = self.output_dir.join("header.rs");
        let mut header_file = File::create(header_path)?;

        writeln!(header_file, "//! Header definition for {}", self.namespace)?;
        writeln!(header_file)?;
        writeln!(header_file, "use super::fields::*;")?;
        writeln!(header_file, "use super::field_enums::*;")?;
        writeln!(header_file)?;

        writeln!(header_file, "/// Standard FIX message header")?;
        writeln!(header_file, "#[derive(Debug, Clone, Default)]")?;
        writeln!(header_file, "pub struct Header {{")?;

        for field_name in &self.header_fields {
            if let Some(field) = self.fields.get(field_name) {
                let field_type = self.rust_type_for_field(&field.field_type);

                match field_name.as_str() {
                    "BeginString" => {
                        writeln!(header_file, "    /// Fixed to BEGIN_STRING constant")?;
                        writeln!(header_file, "    pub begin_string: String,")?;
                    }
                    "BodyLength" => {
                        writeln!(header_file, "    /// Automatically calculated")?;
                        writeln!(header_file, "    pub body_length: u32,")?;
                    }
                    "MsgType" => {
                        writeln!(header_file, "    /// Message type")?;
                        writeln!(header_file, "    pub msg_type: String,")?;
                    }
                    _ => {
                        let field_name_snake = to_snake_case(field_name);
                        writeln!(header_file, "    /// {} field", field_name)?;
                        writeln!(
                            header_file,
                            "    pub {}: Option<{}>,",
                            field_name_snake, field_type
                        )?;
                    }
                }
            }
        }

        writeln!(header_file, "}}")?;
        writeln!(header_file)?;

        // Реализация Header
        writeln!(header_file, "impl Header {{")?;
        writeln!(
            header_file,
            "    /// Create new header with standard fields"
        )?;
        writeln!(header_file, "    pub fn new(msg_type: &str) -> Self {{")?;
        writeln!(header_file, "        Header {{")?;
        writeln!(
            header_file,
            "            begin_string: super::BEGIN_STRING.to_string(),"
        )?;
        writeln!(header_file, "            body_length: 0,")?;
        writeln!(header_file, "            msg_type: msg_type.to_string(),")?;
        writeln!(header_file, "            ..Default::default()")?;
        writeln!(header_file, "        }}")?;
        writeln!(header_file, "    }}")?;
        writeln!(header_file, "    ")?;
        writeln!(header_file, "    /// Convert header to sequence of fields")?;
        writeln!(header_file, "    pub fn to_fields(&self) -> Vec<Field> {{")?;
        writeln!(header_file, "        let mut fields = Vec::new();")?;
        writeln!(header_file, "        ")?;
        writeln!(header_file, "        fields.push(Field::new(FieldTag::BeginString, FieldValue::String(self.begin_string.clone())));")?;
        writeln!(header_file, "        fields.push(Field::new(FieldTag::BodyLength, FieldValue::Int(self.body_length as i64)));")?;
        writeln!(header_file, "        fields.push(Field::new(FieldTag::MsgType, FieldValue::String(self.msg_type.clone())));")?;

        for field_name in &self.header_fields {
            if !vec!["BeginString", "BodyLength", "MsgType"].contains(&field_name.as_str()) {
                let field_name_snake = to_snake_case(field_name);

                if let Some(field) = self.fields.get(field_name) {
                    let field_type = &field.field_type;
                    let field_value = match field_type {
                        t if t == "STRING" => {
                            format!("FieldValue::String(self.{}.clone())", field_name_snake)
                        }
                        t if t == "CHAR" => {
                            format!("FieldValue::Char(self.{})", field_name_snake)
                        }
                        t if t == "INT" => {
                            format!("FieldValue::Int(self.{} as i64)", field_name_snake)
                        }
                        t if t == "FLOAT"
                            || t == "PRICE"
                            || t == "QTY"
                            || t == "AMT"
                            || t == "PERCENTAGE" =>
                        {
                            format!("FieldValue::Float(self.{})", field_name_snake)
                        }
                        t if t == "BOOLEAN" => {
                            format!("FieldValue::Boolean(self.{})", field_name_snake)
                        }
                        t if t == "UTCTIMESTAMP" => {
                            format!("FieldValue::DateTime(self.{}.format(\"%Y%m%d-%H:%M:%S\").to_string())", field_name_snake)
                        }
                        t if t == "UTCDATE" => {
                            format!(
                                "FieldValue::Date(self.{}.format(\"%Y%m%d\").to_string())",
                                field_name_snake
                            )
                        }
                        t if t == "UTCTIME" => {
                            format!(
                                "FieldValue::Time(self.{}.format(\"%H:%M:%S\").to_string())",
                                field_name_snake
                            )
                        }
                        _ => {
                            format!("FieldValue::String(self.{}.to_string())", field_name_snake)
                        }
                    };

                    writeln!(header_file, "        ")?;
                    writeln!(
                        header_file,
                        "        if let Some(value) = &self.{} {{",
                        field_name_snake
                    )?;
                    writeln!(
                        header_file,
                        "            fields.push(Field::new(FieldTag::{}, {}));",
                        field_name, field_value
                    )?;
                    writeln!(header_file, "        }}")?;
                }
            }
        }

        writeln!(header_file, "        ")?;
        writeln!(header_file, "        fields")?;
        writeln!(header_file, "    }}")?;
        writeln!(header_file, "}}")?;

        // Генерируем trailer.rs
        let trailer_path = self.output_dir.join("trailer.rs");
        let mut trailer_file = File::create(trailer_path)?;

        writeln!(
            trailer_file,
            "//! Trailer definition for {}",
            self.namespace
        )?;
        writeln!(trailer_file)?;
        writeln!(trailer_file, "use super::fields::*;")?;
        writeln!(trailer_file, "use super::field_enums::*;")?;
        writeln!(trailer_file)?;

        writeln!(trailer_file, "/// Standard FIX message trailer")?;
        writeln!(trailer_file, "#[derive(Debug, Clone, Default)]")?;
        writeln!(trailer_file, "pub struct Trailer {{")?;

        for field_name in &self.trailer_fields {
            if let Some(field) = self.fields.get(field_name) {
                let field_type = self.rust_type_for_field(&field.field_type);

                if field_name == "CheckSum" {
                    writeln!(trailer_file, "    /// Automatically calculated")?;
                    writeln!(trailer_file, "    pub check_sum: u32,")?;
                } else {
                    let field_name_snake = to_snake_case(field_name);
                    writeln!(trailer_file, "    /// {} field", field_name)?;
                    writeln!(
                        trailer_file,
                        "    pub {}: Option<{}>,",
                        field_name_snake, field_type
                    )?;
                }
            }
        }

        writeln!(trailer_file, "}}")?;
        writeln!(trailer_file)?;

        // Реализация Trailer
        writeln!(trailer_file, "impl Trailer {{")?;
        writeln!(trailer_file, "    /// Create new trailer")?;
        writeln!(trailer_file, "    pub fn new() -> Self {{")?;
        writeln!(trailer_file, "        Trailer {{")?;
        writeln!(trailer_file, "            check_sum: 0,")?;
        writeln!(trailer_file, "            ..Default::default()")?;
        writeln!(trailer_file, "        }}")?;
        writeln!(trailer_file, "    }}")?;
        writeln!(trailer_file, "    ")?;
        writeln!(
            trailer_file,
            "    /// Convert trailer to sequence of fields"
        )?;
        writeln!(trailer_file, "    pub fn to_fields(&self) -> Vec<Field> {{")?;
        writeln!(trailer_file, "        let mut fields = Vec::new();")?;

        for field_name in &self.trailer_fields {
            if field_name != "CheckSum" {
                let field_name_snake = to_snake_case(field_name);

                if let Some(field) = self.fields.get(field_name) {
                    let field_type = &field.field_type;
                    let field_value = match field_type {
                        t if t == "STRING" => {
                            format!("FieldValue::String(self.{}.clone())", field_name_snake)
                        }
                        t if t == "CHAR" => {
                            format!("FieldValue::Char(self.{})", field_name_snake)
                        }
                        t if t == "INT" => {
                            format!("FieldValue::Int(self.{} as i64)", field_name_snake)
                        }
                        t if t == "FLOAT"
                            || t == "PRICE"
                            || t == "QTY"
                            || t == "AMT"
                            || t == "PERCENTAGE" =>
                        {
                            format!("FieldValue::Float(self.{})", field_name_snake)
                        }
                        t if t == "BOOLEAN" => {
                            format!("FieldValue::Boolean(self.{})", field_name_snake)
                        }
                        t if t == "UTCTIMESTAMP" => {
                            format!("FieldValue::DateTime(self.{}.format(\"%Y%m%d-%H:%M:%S\").to_string())", field_name_snake)
                        }
                        t if t == "UTCDATE" => {
                            format!(
                                "FieldValue::Date(self.{}.format(\"%Y%m%d\").to_string())",
                                field_name_snake
                            )
                        }
                        t if t == "UTCTIME" => {
                            format!(
                                "FieldValue::Time(self.{}.format(\"%H:%M:%S\").to_string())",
                                field_name_snake
                            )
                        }
                        _ => {
                            format!("FieldValue::String(self.{}.to_string())", field_name_snake)
                        }
                    };

                    writeln!(trailer_file, "        ")?;
                    writeln!(
                        trailer_file,
                        "        if let Some(value) = &self.{} {{",
                        field_name_snake
                    )?;
                    writeln!(
                        trailer_file,
                        "            fields.push(Field::new(FieldTag::{}, {}));",
                        field_name, field_value
                    )?;
                    writeln!(trailer_file, "        }}")?;
                }
            }
        }

        writeln!(trailer_file, "        ")?;
        writeln!(trailer_file, "        // CheckSum is always last")?;
        writeln!(trailer_file, "        fields.push(Field::new(FieldTag::CheckSum, FieldValue::Int(self.check_sum as i64)));")?;
        writeln!(trailer_file, "        ")?;
        writeln!(trailer_file, "        fields")?;
        writeln!(trailer_file, "    }}")?;
        writeln!(trailer_file, "}}")?;

        Ok(())
    }

    fn generate_components_rs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.output_dir.join("components.rs");
        let mut file = File::create(file_path)?;

        writeln!(file, "//! Component definitions for {}", self.namespace)?;
        writeln!(file)?;
        writeln!(file, "use super::fields::*;")?;
        writeln!(file, "use super::field_enums::*;")?;
        writeln!(file)?;

        for (name, component) in &self.components {
            writeln!(file, "/// {} component", name)?;
            writeln!(file, "#[derive(Debug, Clone, Default)]")?;
            writeln!(file, "pub struct {} {{", name)?;

            // Поля компонента
            for field_name in &component.fields {
                if let Some(field) = self.fields.get(field_name) {
                    let field_type = self.rust_type_for_field(&field.field_type);
                    let field_name_snake = to_snake_case(field_name);

                    writeln!(file, "    /// {} field", field_name)?;
                    writeln!(
                        file,
                        "    pub {}: Option<{}>,",
                        field_name_snake, field_type
                    )?;
                }
            }

            // Группы компонента
            for group in &component.groups {
                let group_name_snake = to_snake_case(&group.name);

                writeln!(file, "    /// {} group", group.name)?;
                writeln!(
                    file,
                    "    pub {}: Vec<{}Group>,",
                    group_name_snake, group.name
                )?;
            }

            // Вложенные компоненты
            for comp_name in &component.components {
                if self.components.contains_key(comp_name) {
                    let comp_name_snake = to_snake_case(comp_name);

                    writeln!(file, "    /// {} component", comp_name)?;
                    writeln!(file, "    pub {}: Option<{}>,", comp_name_snake, comp_name)?;
                }
            }

            writeln!(file, "}}")?;
            writeln!(file)?;

            // Реализация компонента
            writeln!(file, "impl {} {{", name)?;
            writeln!(file, "    /// Create a new {} component", name)?;
            writeln!(file, "    pub fn new() -> Self {{")?;
            writeln!(file, "        Default::default()")?;
            writeln!(file, "    }}")?;
            writeln!(file, "    ")?;
            writeln!(file, "    /// Convert component to sequence of fields")?;
            writeln!(file, "    pub fn to_fields(&self) -> Vec<Field> {{")?;
            writeln!(file, "        let mut fields = Vec::new();")?;

            // Добавление полей компонента
            for field_name in &component.fields {
                if let Some(field) = self.fields.get(field_name) {
                    let field_name_snake = to_snake_case(field_name);
                    let field_type = &field.field_type;
                    let field_value = match field_type {
                        t if t == "STRING" => {
                            format!("FieldValue::String(self.{}.clone())", field_name_snake)
                        }
                        t if t == "CHAR" => {
                            format!("FieldValue::Char(self.{})", field_name_snake)
                        }
                        t if t == "INT" => {
                            format!("FieldValue::Int(self.{} as i64)", field_name_snake)
                        }
                        t if t == "FLOAT"
                            || t == "PRICE"
                            || t == "QTY"
                            || t == "AMT"
                            || t == "PERCENTAGE" =>
                        {
                            format!("FieldValue::Float(self.{})", field_name_snake)
                        }
                        t if t == "BOOLEAN" => {
                            format!("FieldValue::Boolean(self.{})", field_name_snake)
                        }
                        t if t == "UTCTIMESTAMP" => {
                            format!("FieldValue::DateTime(self.{}.format(\"%Y%m%d-%H:%M:%S\").to_string())", field_name_snake)
                        }
                        t if t == "UTCDATE" => {
                            format!(
                                "FieldValue::Date(self.{}.format(\"%Y%m%d\").to_string())",
                                field_name_snake
                            )
                        }
                        t if t == "UTCTIME" => {
                            format!(
                                "FieldValue::Time(self.{}.format(\"%H:%M:%S\").to_string())",
                                field_name_snake
                            )
                        }
                        _ => {
                            format!("FieldValue::String(self.{}.to_string())", field_name_snake)
                        }
                    };

                    writeln!(file, "        ")?;
                    writeln!(
                        file,
                        "        if let Some(value) = &self.{} {{",
                        field_name_snake
                    )?;
                    writeln!(
                        file,
                        "            fields.push(Field::new(FieldTag::{}, {}));",
                        field_name, field_value
                    )?;
                    writeln!(file, "        }}")?;
                }
            }

            // Добавление вложенных компонентов
            for comp_name in &component.components {
                let comp_name_snake = to_snake_case(comp_name);

                writeln!(file, "        ")?;
                writeln!(
                    file,
                    "        if let Some(component) = &self.{} {{",
                    comp_name_snake
                )?;
                writeln!(file, "            fields.extend(component.to_fields());")?;
                writeln!(file, "        }}")?;
            }

            // Добавление групп
            for group in &component.groups {
                let group_name_snake = to_snake_case(&group.name);

                writeln!(file, "        ")?;
                writeln!(file, "        if !self.{}.is_empty() {{", group_name_snake)?;
                writeln!(file, "            // Add NoOf{} field", group.name)?;
                writeln!(file, "            fields.push(Field::new(FieldTag::{}, FieldValue::Int(self.{}.len() as i64)));", group.name, group_name_snake)?;
                writeln!(file, "            ")?;
                writeln!(file, "            // Add each group instance")?;
                writeln!(
                    file,
                    "            for group_instance in &self.{} {{",
                    group_name_snake
                )?;
                writeln!(
                    file,
                    "                fields.extend(group_instance.to_fields());"
                )?;
                writeln!(file, "            }}")?;
                writeln!(file, "        }}")?;
            }

            writeln!(file, "        ")?;
            writeln!(file, "        fields")?;
            writeln!(file, "    }}")?;
            writeln!(file, "}}")?;
            writeln!(file)?;

            // Генерируем группы компонента
            for group in &component.groups {
                self.generate_group(&mut file, group)?;
            }
        }

        Ok(())
    }

    fn generate_group(
        &self,
        file: &mut File,
        group: &Group,
    ) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(file, "/// {} group", group.name)?;
        writeln!(file, "#[derive(Debug, Clone, Default)]")?;
        writeln!(file, "pub struct {}Group {{", group.name)?;

        // Поля группы
        for field_name in &group.fields {
            if let Some(field) = self.fields.get(field_name) {
                let field_type = self.rust_type_for_field(&field.field_type);
                let field_name_snake = to_snake_case(field_name);

                writeln!(file, "    /// {} field", field_name)?;
                writeln!(
                    file,
                    "    pub {}: Option<{}>,",
                    field_name_snake, field_type
                )?;
            }
        }

        // Вложенные компоненты группы
        for comp_name in &group.components {
            if self.components.contains_key(comp_name) {
                let comp_name_snake = to_snake_case(comp_name);

                writeln!(file, "    /// {} component", comp_name)?;
                writeln!(file, "    pub {}: Option<{}>,", comp_name_snake, comp_name)?;
            }
        }

        // Вложенные группы
        for sub_group in &group.sub_groups {
            let group_name_snake = to_snake_case(&sub_group.name);

            writeln!(file, "    /// {} group", sub_group.name)?;
            writeln!(
                file,
                "    pub {}: Vec<{}Group>,",
                group_name_snake, sub_group.name
            )?;
        }

        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация группы
        writeln!(file, "impl {}Group {{", group.name)?;
        writeln!(file, "    /// Create a new {} group", group.name)?;
        writeln!(file, "    pub fn new() -> Self {{")?;
        writeln!(file, "        Default::default()")?;
        writeln!(file, "    }}")?;
        writeln!(file, "    ")?;
        writeln!(file, "    /// Convert group to sequence of fields")?;
        writeln!(file, "    pub fn to_fields(&self) -> Vec<Field> {{")?;
        writeln!(file, "        let mut fields = Vec::new();")?;

        // Добавление полей группы
        for field_name in &group.fields {
            if let Some(field) = self.fields.get(field_name) {
                let field_name_snake = to_snake_case(field_name);
                let field_type = &field.field_type;
                let field_value = match field_type {
                    t if t == "STRING" => {
                        format!("FieldValue::String(self.{}.clone())", field_name_snake)
                    }
                    t if t == "CHAR" => {
                        format!("FieldValue::Char(self.{})", field_name_snake)
                    }
                    t if t == "INT" => {
                        format!("FieldValue::Int(self.{} as i64)", field_name_snake)
                    }
                    t if t == "FLOAT"
                        || t == "PRICE"
                        || t == "QTY"
                        || t == "AMT"
                        || t == "PERCENTAGE" =>
                    {
                        format!("FieldValue::Float(self.{})", field_name_snake)
                    }
                    t if t == "BOOLEAN" => {
                        format!("FieldValue::Boolean(self.{})", field_name_snake)
                    }
                    t if t == "UTCTIMESTAMP" => {
                        format!(
                            "FieldValue::DateTime(self.{}.format(\"%Y%m%d-%H:%M:%S\").to_string())",
                            field_name_snake
                        )
                    }
                    t if t == "UTCDATE" => {
                        format!(
                            "FieldValue::Date(self.{}.format(\"%Y%m%d\").to_string())",
                            field_name_snake
                        )
                    }
                    t if t == "UTCTIME" => {
                        format!(
                            "FieldValue::Time(self.{}.format(\"%H:%M:%S\").to_string())",
                            field_name_snake
                        )
                    }
                    _ => {
                        format!("FieldValue::String(self.{}.to_string())", field_name_snake)
                    }
                };

                writeln!(file, "        ")?;
                writeln!(
                    file,
                    "        if let Some(value) = &self.{} {{",
                    field_name_snake
                )?;
                writeln!(
                    file,
                    "            fields.push(Field::new(FieldTag::{}, {}));",
                    field_name, field_value
                )?;
                writeln!(file, "        }}")?;
            }
        }

        // Добавление вложенных компонентов
        for comp_name in &group.components {
            let comp_name_snake = to_snake_case(comp_name);

            writeln!(file, "        ")?;
            writeln!(
                file,
                "        if let Some(component) = &self.{} {{",
                comp_name_snake
            )?;
            writeln!(file, "            fields.extend(component.to_fields());")?;
            writeln!(file, "        }}")?;
        }

        // Добавление вложенных групп
        for sub_group in &group.sub_groups {
            let group_name_snake = to_snake_case(&sub_group.name);

            writeln!(file, "        ")?;
            writeln!(file, "        if !self.{}.is_empty() {{", group_name_snake)?;
            writeln!(file, "            // Add NoOf{} field", sub_group.name)?;
            writeln!(file, "            fields.push(Field::new(FieldTag::{}, FieldValue::Int(self.{}.len() as i64)));", sub_group.name, group_name_snake)?;
            writeln!(file, "            ")?;
            writeln!(file, "            // Add each group instance")?;
            writeln!(
                file,
                "            for group_instance in &self.{} {{",
                group_name_snake
            )?;
            writeln!(
                file,
                "                fields.extend(group_instance.to_fields());"
            )?;
            writeln!(file, "            }}")?;
            writeln!(file, "        }}")?;
        }

        writeln!(file, "        ")?;
        writeln!(file, "        fields")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Генерируем вложенные группы
        for sub_group in &group.sub_groups {
            self.generate_group(file, sub_group)?;
        }

        Ok(())
    }

    fn generate_messages_rs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.output_dir.join("messages.rs");
        let mut file = File::create(file_path)?;

        writeln!(file, "//! Message definitions for {}", self.namespace)?;
        writeln!(file)?;
        writeln!(file, "use super::fields::*;")?;
        writeln!(file, "use super::field_enums::*;")?;
        writeln!(file, "use super::header::Header;")?;
        writeln!(file, "use super::trailer::Trailer;")?;
        writeln!(file, "use super::components::*;")?;
        writeln!(file)?;

        // Генерируем перечисление для типов сообщений
        writeln!(file, "/// Available message types")?;
        writeln!(file, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
        writeln!(file, "pub enum MessageType {{")?;

        for msg in &self.messages {
            writeln!(file, "    /// {} message", msg.name)?;
            writeln!(file, "    {},", msg.name)?;
        }

        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация MessageType
        writeln!(file, "impl MessageType {{")?;
        writeln!(file, "    /// Convert message type to FIX type string")?;
        writeln!(file, "    pub fn to_string(&self) -> String {{")?;
        writeln!(file, "        match self {{")?;

        for msg in &self.messages {
            writeln!(
                file,
                "            MessageType::{} => \"{}\".to_string(),",
                msg.name, msg.msgtype
            )?;
        }

        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "    ")?;
        writeln!(file, "    /// Try to convert a string to a message type")?;
        writeln!(file, "    pub fn from_string(s: &str) -> Option<Self> {{")?;
        writeln!(file, "        match s {{")?;

        for msg in &self.messages {
            writeln!(
                file,
                "            \"{}\" => Some(MessageType::{}),",
                msg.msgtype, msg.name
            )?;
        }

        writeln!(file, "            _ => None,")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Генерируем базовую структуру Message
        writeln!(file, "/// Base FIX message")?;
        writeln!(file, "#[derive(Debug, Clone)]")?;
        writeln!(file, "pub struct Message {{")?;
        writeln!(file, "    /// Message header")?;
        writeln!(file, "    pub header: Header,")?;
        writeln!(file, "    /// Message body fields")?;
        writeln!(file, "    pub fields: Vec<Field>,")?;
        writeln!(file, "    /// Message trailer")?;
        writeln!(file, "    pub trailer: Trailer,")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Реализация Message
        writeln!(file, "impl Message {{")?;
        writeln!(
            file,
            "    /// Create a new empty message with the given message type"
        )?;
        writeln!(file, "    pub fn new(msg_type: MessageType) -> Self {{")?;
        writeln!(file, "        Message {{")?;
        writeln!(
            file,
            "            header: Header::new(&msg_type.to_string()),"
        )?;
        writeln!(file, "            fields: Vec::new(),")?;
        writeln!(file, "            trailer: Trailer::new(),")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "    ")?;
        writeln!(file, "    /// Encode the message as a FIX string")?;
        writeln!(file, "    pub fn to_fix_string(&self) -> String {{")?;
        writeln!(file, "        let mut result = String::new();")?;
        writeln!(file, "        ")?;
        writeln!(
            file,
            "        // Convert message body to string first to calculate body length"
        )?;
        writeln!(file, "        let mut body = String::new();")?;
        writeln!(file, "        for field in &self.fields {{")?;
        writeln!(file, "            body.push_str(&field.to_fix_string());")?;
        writeln!(file, "        }}")?;
        writeln!(file, "        ")?;
        writeln!(
            file,
            "        // Create a mutable copy of the header with the correct body length"
        )?;
        writeln!(file, "        let mut header = self.header.clone();")?;
        writeln!(file, "        header.body_length = body.len() as u32;")?;
        writeln!(file, "        ")?;
        writeln!(file, "        // Add header fields")?;
        writeln!(file, "        for field in header.to_fields() {{")?;
        writeln!(file, "            result.push_str(&field.to_fix_string());")?;
        writeln!(file, "        }}")?;
        writeln!(file, "        ")?;
        writeln!(file, "        // Add body")?;
        writeln!(file, "        result.push_str(&body);")?;
        writeln!(file, "        ")?;
        writeln!(file, "        // Calculate checksum")?;
        writeln!(
            file,
            "        let checksum = result.bytes().fold(0u32, |sum, b| (sum + b as u32) % 256);"
        )?;
        writeln!(file, "        ")?;
        writeln!(
            file,
            "        // Create a mutable copy of the trailer with the correct checksum"
        )?;
        writeln!(file, "        let mut trailer = self.trailer.clone();")?;
        writeln!(file, "        trailer.check_sum = checksum;")?;
        writeln!(file, "        ")?;
        writeln!(file, "        // Add trailer fields")?;
        writeln!(file, "        for field in trailer.to_fields() {{")?;
        writeln!(file, "            result.push_str(&field.to_fix_string());")?;
        writeln!(file, "        }}")?;
        writeln!(file, "        ")?;
        writeln!(file, "        result")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
        writeln!(file)?;

        // Генерируем структуры для каждого типа сообщения
        for msg in &self.messages {
            writeln!(file, "/// {} message", msg.name)?;
            writeln!(file, "#[derive(Debug, Clone, Default)]")?;
            writeln!(file, "pub struct {} {{", msg.name)?;

            // Поля сообщения
            for field_name in &msg.fields {
                if let Some(field) = self.fields.get(field_name) {
                    let field_type = self.rust_type_for_field(&field.field_type);
                    let field_name_snake = to_snake_case(field_name);

                    // Если поле обязательное, то используем тип, иначе Option<тип>
                    if msg.required_fields.contains(field_name) {
                        writeln!(file, "    /// {} field (required)", field_name)?;
                        writeln!(file, "    pub {}: {},", field_name_snake, field_type)?;
                    } else {
                        writeln!(file, "    /// {} field", field_name)?;
                        writeln!(
                            file,
                            "    pub {}: Option<{}>,",
                            field_name_snake, field_type
                        )?;
                    }
                }
            }

            // Компоненты сообщения
            for comp_name in &msg.components {
                if self.components.contains_key(comp_name) {
                    let comp_name_snake = to_snake_case(comp_name);

                    writeln!(file, "    /// {} component", comp_name)?;
                    writeln!(file, "    pub {}: Option<{}>,", comp_name_snake, comp_name)?;
                }
            }

            // Группы сообщения
            for group in &msg.groups {
                let group_name_snake = to_snake_case(&group.name);

                writeln!(file, "    /// {} group", group.name)?;
                writeln!(
                    file,
                    "    pub {}: Vec<{}Group>,",
                    group_name_snake, group.name
                )?;
            }

            writeln!(file, "}}")?;
            writeln!(file)?;

            // Реализация конкретного сообщения
            writeln!(file, "impl {} {{", msg.name)?;
            writeln!(file, "    /// Create a new {} message", msg.name)?;

            // Конструктор с обязательными полями (если есть)
            if !msg.required_fields.is_empty() {
                writeln!(file, "    pub fn new(")?;
                for (i, field_name) in msg.required_fields.iter().enumerate() {
                    if let Some(field) = self.fields.get(field_name) {
                        let field_type = self.rust_type_for_field(&field.field_type);
                        let field_name_snake = to_snake_case(field_name);

                        if i < msg.required_fields.len() - 1 {
                            writeln!(file, "        {}: {},", field_name_snake, field_type)?;
                        } else {
                            writeln!(file, "        {}: {}", field_name_snake, field_type)?;
                        }
                    }
                }
                writeln!(file, "    ) -> Self {{")?;
                writeln!(file, "        let mut msg = Self::default();")?;

                for field_name in &msg.required_fields {
                    let field_name_snake = to_snake_case(field_name);
                    writeln!(
                        file,
                        "        msg.{} = {};",
                        field_name_snake, field_name_snake
                    )?;
                }

                writeln!(file, "        msg")?;
                writeln!(file, "    }}")?;
            } else {
                writeln!(file, "    pub fn new() -> Self {{")?;
                writeln!(file, "        Self::default()")?;
                writeln!(file, "    }}")?;
            }

            writeln!(file, "    ")?;
            writeln!(file, "    /// Convert to base Message type")?;
            writeln!(file, "    pub fn to_message(&self) -> Message {{")?;
            writeln!(
                file,
                "        let mut message = Message::new(MessageType::{});",
                msg.name
            )?;
            writeln!(file, "        ")?;

            // Добавление полей сообщения
            for field_name in &msg.fields {
                if let Some(field) = self.fields.get(field_name) {
                    let field_name_snake = to_snake_case(field_name);
                    let field_type = &field.field_type;

                    if msg.required_fields.contains(field_name) {
                        // Обязательное поле
                        let field_value = match field_type {
                            t if t == "STRING" => {
                                format!("FieldValue::String(self.{}.clone())", field_name_snake)
                            }
                            t if t == "CHAR" => {
                                format!("FieldValue::Char(self.{})", field_name_snake)
                            }
                            t if t == "INT" => {
                                format!("FieldValue::Int(self.{} as i64)", field_name_snake)
                            }
                            t if t == "FLOAT"
                                || t == "PRICE"
                                || t == "QTY"
                                || t == "AMT"
                                || t == "PERCENTAGE" =>
                            {
                                format!("FieldValue::Float(self.{})", field_name_snake)
                            }
                            t if t == "BOOLEAN" => {
                                format!("FieldValue::Boolean(self.{})", field_name_snake)
                            }
                            t if t == "UTCTIMESTAMP" => {
                                format!("FieldValue::DateTime(self.{}.format(\"%Y%m%d-%H:%M:%S\").to_string())", field_name_snake)
                            }
                            t if t == "UTCDATE" => {
                                format!(
                                    "FieldValue::Date(self.{}.format(\"%Y%m%d\").to_string())",
                                    field_name_snake
                                )
                            }
                            t if t == "UTCTIME" => {
                                format!(
                                    "FieldValue::Time(self.{}.format(\"%H:%M:%S\").to_string())",
                                    field_name_snake
                                )
                            }
                            _ => {
                                format!("FieldValue::String(self.{}.to_string())", field_name_snake)
                            }
                        };

                        writeln!(
                            file,
                            "        message.fields.push(Field::new(FieldTag::{}, {}));",
                            field_name, field_value
                        )?;
                    } else {
                        // Необязательное поле
                        let field_value = match field_type {
                            t if t == "STRING" => {
                                format!("FieldValue::String(value.clone())")
                            }
                            t if t == "CHAR" => {
                                format!("FieldValue::Char(*value)")
                            }
                            t if t == "INT" => {
                                format!("FieldValue::Int(*value as i64)")
                            }
                            t if t == "FLOAT"
                                || t == "PRICE"
                                || t == "QTY"
                                || t == "AMT"
                                || t == "PERCENTAGE" =>
                            {
                                format!("FieldValue::Float(*value)")
                            }
                            t if t == "BOOLEAN" => {
                                format!("FieldValue::Boolean(*value)")
                            }
                            t if t == "UTCTIMESTAMP" => {
                                format!("FieldValue::DateTime(value.format(\"%Y%m%d-%H:%M:%S\").to_string())")
                            }
                            t if t == "UTCDATE" => {
                                format!("FieldValue::Date(value.format(\"%Y%m%d\").to_string())")
                            }
                            t if t == "UTCTIME" => {
                                format!("FieldValue::Time(value.format(\"%H:%M:%S\").to_string())")
                            }
                            _ => {
                                format!("FieldValue::String(value.to_string())")
                            }
                        };

                        writeln!(
                            file,
                            "        if let Some(value) = &self.{} {{",
                            field_name_snake
                        )?;
                        writeln!(
                            file,
                            "            message.fields.push(Field::new(FieldTag::{}, {}));",
                            field_name, field_value
                        )?;
                        writeln!(file, "        }}")?;
                    }
                }
            }

            // Добавление компонентов
            for comp_name in &msg.components {
                let comp_name_snake = to_snake_case(comp_name);

                writeln!(file, "        ")?;
                writeln!(
                    file,
                    "        if let Some(component) = &self.{} {{",
                    comp_name_snake
                )?;
                writeln!(
                    file,
                    "            message.fields.extend(component.to_fields());"
                )?;
                writeln!(file, "        }}")?;
            }

            // Добавление групп
            for group in &msg.groups {
                let group_name_snake = to_snake_case(&group.name);

                writeln!(file, "        ")?;
                writeln!(file, "        if !self.{}.is_empty() {{", group_name_snake)?;
                writeln!(file, "            // Add NoOf{} field", group.name)?;
                writeln!(file, "            message.fields.push(Field::new(FieldTag::{}, FieldValue::Int(self.{}.len() as i64)));", group.name, group_name_snake)?;
                writeln!(file, "            ")?;
                writeln!(file, "            // Add each group instance")?;
                writeln!(
                    file,
                    "            for group_instance in &self.{} {{",
                    group_name_snake
                )?;
                writeln!(
                    file,
                    "                message.fields.extend(group_instance.to_fields());"
                )?;
                writeln!(file, "            }}")?;
                writeln!(file, "        }}")?;
            }

            writeln!(file, "        ")?;
            writeln!(file, "        message")?;
            writeln!(file, "    }}")?;
            writeln!(file, "}}")?;
            writeln!(file)?;

            // Генерируем группы сообщения
            for group in &msg.groups {
                self.generate_group(&mut file, group)?;
            }
        }

        Ok(())
    }

    fn generate_field_enums_rs(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.output_dir.join("field_enums.rs");
        let mut file = File::create(file_path)?;

        writeln!(file, "//! Field value enumerations for {}", self.namespace)?;
        writeln!(file)?;

        // Для каждого поля с перечислениями значений создаем enum
        for field in self.fields.values() {
            if !field.values.is_empty() {
                let field_type = match field.field_type.as_str() {
                    "STRING" => "String",
                    "CHAR" => "char",
                    "INT" => "i64",
                    "FLOAT" | "PRICE" | "QTY" | "AMT" | "PERCENTAGE" => "f64",
                    "BOOLEAN" => "bool",
                    _ => "String",
                };

                writeln!(file, "/// Values for {} field", field.name)?;
                writeln!(file, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
                writeln!(file, "pub enum {} {{", field.name)?;

                for (description, _enum_value) in &field.values {
                    // Преобразуем описание в валидный идентификатор Rust
                    let enum_variant = description
                        .replace(" ", "")
                        .replace("-", "")
                        .replace("/", "")
                        .replace("(", "")
                        .replace(")", "")
                        .replace(".", "")
                        .replace(",", "")
                        .replace("'", "")
                        .replace("\"", "");

                    writeln!(file, "    /// {}", description)?;
                    writeln!(file, "    {},", enum_variant)?;
                }

                writeln!(file, "}}")?;
                writeln!(file)?;

                // Реализация методов для enum
                writeln!(file, "impl {} {{", field.name)?;
                writeln!(file, "    /// Convert enum to its FIX wire value")?;
                writeln!(file, "    pub fn to_fix_value(&self) -> {} {{", field_type)?;
                writeln!(file, "        match self {{")?;

                for (description, enum_value) in &field.values {
                    let enum_variant = description
                        .replace(" ", "")
                        .replace("-", "")
                        .replace("/", "")
                        .replace("(", "")
                        .replace(")", "")
                        .replace(".", "")
                        .replace(",", "")
                        .replace("'", "")
                        .replace("\"", "");

                    match field.field_type.as_str() {
                        "STRING" => {
                            writeln!(
                                file,
                                "            {}::{} => \"{}\".to_string(),",
                                field.name, enum_variant, enum_value
                            )?;
                        }
                        "CHAR" => {
                            writeln!(
                                file,
                                "            {}::{} => '{}',",
                                field.name, enum_variant, enum_value
                            )?;
                        }
                        "INT" => {
                            writeln!(
                                file,
                                "            {}::{} => {},",
                                field.name, enum_variant, enum_value
                            )?;
                        }
                        "FLOAT" | "PRICE" | "QTY" | "AMT" | "PERCENTAGE" => {
                            writeln!(
                                file,
                                "            {}::{} => {}.0,",
                                field.name, enum_variant, enum_value
                            )?;
                        }
                        "BOOLEAN" => {
                            let bool_value = if enum_value == "Y" { "true" } else { "false" };
                            writeln!(
                                file,
                                "            {}::{} => {},",
                                field.name, enum_variant, bool_value
                            )?;
                        }
                        _ => {
                            writeln!(
                                file,
                                "            {}::{} => \"{}\".to_string(),",
                                field.name, enum_variant, enum_value
                            )?;
                        }
                    }
                }

                writeln!(file, "        }}")?;
                writeln!(file, "    }}")?;
                writeln!(file, "    ")?;
                writeln!(file, "    /// Try to convert a FIX value to enum")?;
                writeln!(
                    file,
                    "    pub fn from_fix_value(value: {}) -> Option<Self> {{",
                    field_type
                )?;
                writeln!(file, "        match value {{")?;

                for (description, enum_value) in &field.values {
                    let enum_variant = description
                        .replace(" ", "")
                        .replace("-", "")
                        .replace("/", "")
                        .replace("(", "")
                        .replace(")", "")
                        .replace(".", "")
                        .replace(",", "")
                        .replace("'", "")
                        .replace("\"", "");

                    match field.field_type.as_str() {
                        "STRING" => {
                            writeln!(
                                file,
                                "            s if s == \"{}\" => Some({}::{}),",
                                enum_value, field.name, enum_variant
                            )?;
                        }
                        "CHAR" => {
                            writeln!(
                                file,
                                "            c if c == '{}' => Some({}::{}),",
                                enum_value, field.name, enum_variant
                            )?;
                        }
                        "INT" => {
                            writeln!(
                                file,
                                "            i if i == {} => Some({}::{}),",
                                enum_value, field.name, enum_variant
                            )?;
                        }
                        "FLOAT" | "PRICE" | "QTY" | "AMT" | "PERCENTAGE" => {
                            writeln!(
                                file,
                                "            f if f == {}.0 => Some({}::{}),",
                                enum_value, field.name, enum_variant
                            )?;
                        }
                        "BOOLEAN" => {
                            let bool_value = if enum_value == "Y" { "true" } else { "false" };
                            writeln!(
                                file,
                                "            b if b == {} => Some({}::{}),",
                                bool_value, field.name, enum_variant
                            )?;
                        }
                        _ => {
                            writeln!(
                                file,
                                "            s if s == \"{}\" => Some({}::{}),",
                                enum_value, field.name, enum_variant
                            )?;
                        }
                    }
                }

                writeln!(file, "            _ => None,")?;
                writeln!(file, "        }}")?;
                writeln!(file, "    }}")?;
                writeln!(file, "}}")?;
                writeln!(file)?;
            }
        }

        Ok(())
    }

    fn rust_type_for_field(&self, field_type: &str) -> String {
        match field_type {
            "STRING" => "String".to_string(),
            "CHAR" => "char".to_string(),
            "INT" => "i64".to_string(),
            "FLOAT" | "PRICE" | "QTY" | "AMT" | "PERCENTAGE" => "f64".to_string(),
            "BOOLEAN" => "bool".to_string(),
            "UTCTIMESTAMP" | "UTCDATE" | "UTCTIME" => "chrono::DateTime<chrono::Utc>".to_string(),
            "DATA" => "Vec<u8>".to_string(),
            _ => "String".to_string(),
        }
    }
}

// Функция для преобразования CamelCase в snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty()
                && !result.ends_with('_')
                && (chars.peek().map_or(false, |next| next.is_lowercase())
                    || result
                        .chars()
                        .last()
                        .map_or(false, |prev| prev.is_lowercase()))
            {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Обработка аргументов командной строки
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <xml_file> <output_dir>", args[0]);
        process::exit(1);
    }

    let xml_file = &args[1];
    let output_dir = Path::new(&args[2]);

    // Создаем и запускаем процессор
    let mut processor = FixProcessor::new(xml_file, output_dir)?;
    processor.process(xml_file)?;

    println!("Successfully generated Rust code for {}", xml_file);

    Ok(())
}
