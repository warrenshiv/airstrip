#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Department struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Department {
    id: u64,
    name: String,
    description: String,
    created_at: u64,
}

// Doctor struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Doctor {
    id: u64,
    name: String,
    department_id: u64,
    image: String,
    is_available: bool,
}

// Patient struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Patient {
    id: u64,
    name: String,
    age: u64,
    gender: String,
    phone_number: String,
    email: String,
    address: String,
    emergency_contact: EmergencyContact,
    allergies: Vec<String>,
    current_medications: Vec<String>,
    medical_history: Vec<String>,
}

// EmergencyContact struct for Patient
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EmergencyContact {
    name: String,
    phone_number: String,
    relationship: String,
}

// Consultation struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Consultation {
    id: u64,
    patient_id: u64,
    problem: String,
    department_id: u64,
}

// Chat struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Chat {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    message: String,
    timestamp: u64,
}

// Appointment struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Appointment {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    reason: String,
    appointment_time: u64,
    status: String,             // "scheduled", "canceled", "completed"
    video_link: Option<String>, // Optional link for video conferencing
}

// Prescription struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Prescription {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    medications: Vec<String>,
    instructions: String,
    issued_at: u64,
}

// Payment struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Payment {
    id: u64,
    appointment_id: u64,
    patient_id: u64,
    amount: u64,
    status: String,         // "pending", "completed", "failed"
    payment_method: String, // "credit_card", "paypal"
}

// MedicalRecord struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MedicalRecord {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    prescriptions: Vec<Prescription>,
    lab_results: Vec<String>,
    diagnosis: String,
    treatment: String,
    created_at: u64,
}

// Message enum
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    PaymentFailed(String),
    PaymentCompleted(String),
}

// Payload Structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateDepartmentPayload {
    name: String,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateDoctorPayload {
    name: String,
    department_id: u64,
    image: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreatePatientPayload {
    name: String,
    age: u64,
    gender: String,
    phone_number: String,
    email: String,
    address: String,
    emergency_contact: EmergencyContact,
    allergies: Vec<String>,
    current_medications: Vec<String>,
    medical_history: Vec<String>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateConsultationPayload {
    patient_id: u64,
    problem: String,
    department_id: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateChatPayload {
    patient_id: u64,
    doctor_id: u64,
    message: String,
    timestamp: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateAppointmentPayload {
    patient_id: u64,
    doctor_id: u64,
    reason: String,
    appointment_time: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreatePrescriptionPayload {
    patient_id: u64,
    doctor_id: u64,
    medications: Vec<String>,
    instructions: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreatePaymentPayload {
    patient_id: u64,
    appointment_id: u64,
    amount: u64,
    payment_method: String,
}

// Create patient medical record payload
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateMedicalRecordPayload {
    patient_id: u64,
    doctor_id: u64,
    prescriptions: Vec<Prescription>,
    lab_results: Vec<String>,
    diagnosis: String,
    treatment: String,
}

// Implementing Storable for department
impl Storable for Department {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for department
impl BoundedStorable for Department {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for doctor
impl Storable for Doctor {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for doctor
impl BoundedStorable for Doctor {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for patient
impl Storable for Patient {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for patient
impl BoundedStorable for Patient {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for consultation
impl Storable for Consultation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for consultation
impl BoundedStorable for Consultation {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for chat
impl Storable for Chat {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for chat
impl BoundedStorable for Chat {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for appointment
impl Storable for Appointment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for appointment
impl BoundedStorable for Appointment {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for prescription
impl Storable for Prescription {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for prescription
impl BoundedStorable for Prescription {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for payment
impl Storable for Payment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for payment
impl BoundedStorable for Payment {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for medical record
impl Storable for MedicalRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for medical record
impl BoundedStorable for MedicalRecord {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for emergency contact
impl Storable for EmergencyContact {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for emergency contact
impl BoundedStorable for EmergencyContact {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for message
impl Storable for Message {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implementing BoundedStorable for message
impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management using thread_local
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static DEPARTMENTS: RefCell<StableBTreeMap<u64, Department, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static DOCTORS: RefCell<StableBTreeMap<u64, Doctor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static PATIENTS: RefCell<StableBTreeMap<u64, Patient, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static CONSULTATIONS: RefCell<StableBTreeMap<u64, Consultation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static CHATS: RefCell<StableBTreeMap<u64, Chat, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static APPOINTMENTS: RefCell<StableBTreeMap<u64, Appointment, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static PRESCRIPTIONS: RefCell<StableBTreeMap<u64, Prescription, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));

    static PAYMENTS: RefCell<StableBTreeMap<u64, Payment, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8)))
    ));

    static MEDICAL_RECORDS: RefCell<StableBTreeMap<u64, MedicalRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9)))
    ));
}

// Function for creating a department
#[ic_cdk::update]
fn create_department(payload: CreateDepartmentPayload) -> Result<Department, Message> {
    if payload.name.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    let department_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let department = Department {
        id: department_id,
        name: payload.name,
        description: payload.description,
        created_at: time(),
    };

    DEPARTMENTS.with(|departments| {
        departments
            .borrow_mut()
            .insert(department_id, department.clone())
    });
    Ok(department)
}

// FUnction to get a department by id
#[ic_cdk::query]
fn get_department_by_id(id: u64) -> Result<Department, Message> {
    DEPARTMENTS.with(|departments| match departments.borrow().get(&id) {
        Some(department) => Ok(department.clone()),
        None => Err(Message::NotFound("Department not found".to_string())),
    })
}

// Function to get all departments
#[ic_cdk::query]
fn get_all_departments() -> Result<Vec<Department>, Message> {
    DEPARTMENTS.with(|storage| {
        let departments: Vec<Department> = storage
            .borrow()
            .iter()
            .map(|(_, department)| department.clone())
            .collect();

        if departments.is_empty() {
            Err(Message::NotFound("No departments found".to_string()))
        } else {
            Ok(departments)
        }
    })
}

// Search departments by name
#[ic_cdk::query]
fn search_departments_by_name(name: String) -> Result<Vec<Department>, Message> {
    DEPARTMENTS.with(|storage| {
        let departments: Vec<Department> = storage
            .borrow()
            .iter()
            .filter(|(_, department)| department.name.contains(&name))
            .map(|(_, department)| department.clone())
            .collect();

        if departments.is_empty() {
            Err(Message::NotFound("No departments found".to_string()))
        } else {
            Ok(departments)
        }
    })
}

// Function to create a doctor
#[ic_cdk::update]
fn create_doctor(payload: CreateDoctorPayload) -> Result<Doctor, Message> {
    if payload.name.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Check if the department exists and return a Result
    let department_exists =
        DEPARTMENTS.with(|departments| departments.borrow().contains_key(&payload.department_id));

    if !department_exists {
        return Err(Message::NotFound("Department not found".to_string()));
    }

    let doctor_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let doctor = Doctor {
        id: doctor_id,
        name: payload.name,
        department_id: payload.department_id,
        image: payload.image,
        is_available: true,
    };

    DOCTORS.with(|doctors| doctors.borrow_mut().insert(doctor_id, doctor.clone()));
    Ok(doctor)
}

// Function to get a doctor by id
#[ic_cdk::query]
fn get_doctor_by_id(id: u64) -> Result<Doctor, Message> {
    DOCTORS.with(|doctors| match doctors.borrow().get(&id) {
        Some(doctor) => Ok(doctor.clone()),
        None => Err(Message::NotFound("Doctor not found".to_string())),
    })
}

// Function to get all doctors
#[ic_cdk::query]
fn get_all_doctors() -> Result<Vec<Doctor>, Message> {
    DOCTORS.with(|storage| {
        let doctors: Vec<Doctor> = storage
            .borrow()
            .iter()
            .map(|(_, doctor)| doctor.clone())
            .collect();

        if doctors.is_empty() {
            Err(Message::NotFound("No doctors found".to_string()))
        } else {
            Ok(doctors)
        }
    })
}

// Function to get all doctors by department id
#[ic_cdk::query]
fn get_doctors_by_department_id(department_id: u64) -> Result<Vec<Doctor>, Message> {
    DOCTORS.with(|storage| {
        let doctors: Vec<Doctor> = storage
            .borrow()
            .iter()
            .filter(|(_, doctor)| doctor.department_id == department_id)
            .map(|(_, doctor)| doctor.clone())
            .collect();

        if doctors.is_empty() {
            Err(Message::NotFound("No doctors found".to_string()))
        } else {
            Ok(doctors)
        }
    })
}

// Function to update doctor Availability
#[ic_cdk::update]
fn update_doctor_availability(doctor_id: u64, is_available: bool) -> Result<Doctor, Message> {
    DOCTORS.with(|doctors| {
        // Get mutable access to the inner StableBTreeMap
        let mut doctors_map = doctors.borrow_mut();

        // Try to find and update the doctor
        if let Some(doctor) = doctors_map.get(&doctor_id) {
            // Create updated doctor with new availability
            let mut updated_doctor = doctor.clone();
            updated_doctor.is_available = is_available;

            // Insert the updated doctor back into the map
            match doctors_map.insert(doctor_id, updated_doctor.clone()) {
                Some(_) => Ok(updated_doctor),
                None => Ok(updated_doctor),
            }
        } else {
            Err(Message::NotFound("Doctor not found".to_string()))
        }
    })
}

// Search doctors by name
#[ic_cdk::query]
fn search_doctors_by_name(name: String) -> Result<Vec<Doctor>, Message> {
    DOCTORS.with(|storage| {
        let doctors: Vec<Doctor> = storage
            .borrow()
            .iter()
            .filter(|(_, doctor)| doctor.name.contains(&name))
            .map(|(_, doctor)| doctor.clone())
            .collect();

        if doctors.is_empty() {
            Err(Message::NotFound("No doctors found".to_string()))
        } else {
            Ok(doctors)
        }
    })
}

// Function to create a patient
#[ic_cdk::update]
fn create_patient(payload: CreatePatientPayload) -> Result<Patient, Message> {
    // Check if required fields are present
    if payload.name.is_empty() || payload.phone_number.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Generate a new patient ID by incrementing the ID counter
    let patient_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new Patient struct from the provided payload
    let patient = Patient {
        id: patient_id,
        name: payload.name,
        age: payload.age,
        gender: payload.gender,
        phone_number: payload.phone_number,
        email: payload.email,
        address: payload.address,
        emergency_contact: payload.emergency_contact,
        allergies: payload.allergies,
        current_medications: payload.current_medications,
        medical_history: payload.medical_history,
    };

    // Insert the new patient into the PATIENTS stable storage
    PATIENTS.with(|patients| {
        patients.borrow_mut().insert(patient_id, patient.clone());
    });

    // Return the newly created patient
    Ok(patient)
}

// Function to get a patient by id
#[ic_cdk::query]
fn get_patient_by_id(id: u64) -> Result<Patient, Message> {
    PATIENTS.with(|patients| match patients.borrow().get(&id) {
        Some(patient) => Ok(patient.clone()),
        None => Err(Message::NotFound("Patient not found".to_string())),
    })
}

// Function to get all patients
#[ic_cdk::query]
fn get_all_patients() -> Result<Vec<Patient>, Message> {
    PATIENTS.with(|storage| {
        let patients: Vec<Patient> = storage
            .borrow()
            .iter()
            .map(|(_, patient)| patient.clone())
            .collect();

        if patients.is_empty() {
            Err(Message::NotFound("No patients found".to_string()))
        } else {
            Ok(patients)
        }
    })
}

// Function to create a consultation
#[ic_cdk::update]
fn create_consultation(payload: CreateConsultationPayload) -> Result<Consultation, Message> {
    // Check if required fields are present
    if payload.patient_id == 0 || payload.problem.is_empty() || payload.department_id == 0 {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Check if the patient exists
    let patient_exists =
        PATIENTS.with(|patients| patients.borrow().contains_key(&payload.patient_id));
    if !patient_exists {
        return Err(Message::NotFound("Patient not found".to_string()));
    }

    // Check if the department exists
    let department_exists =
        DEPARTMENTS.with(|departments| departments.borrow().contains_key(&payload.department_id));
    if !department_exists {
        return Err(Message::NotFound("Department not found".to_string()));
    }

    // Generate a new consultation ID by incrementing the ID counter
    let consultation_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new Consultation struct from the provided payload
    let consultation = Consultation {
        id: consultation_id,
        patient_id: payload.patient_id,
        problem: payload.problem,
        department_id: payload.department_id,
    };

    // Insert the new consultation into the CONSULTATIONS stable storage
    CONSULTATIONS.with(|consultations| {
        consultations
            .borrow_mut()
            .insert(consultation_id, consultation.clone());
    });

    // Return the newly created consultation
    Ok(consultation)
}

// Function to get a consultation by id
#[ic_cdk::query]
fn get_consultation_by_id(id: u64) -> Result<Consultation, Message> {
    CONSULTATIONS.with(|consultations| match consultations.borrow().get(&id) {
        Some(consultation) => Ok(consultation.clone()),
        None => Err(Message::NotFound("Consultation not found".to_string())),
    })
}

// Function to get all consultations
#[ic_cdk::query]
fn get_all_consultations() -> Result<Vec<Consultation>, Message> {
    CONSULTATIONS.with(|storage| {
        let consultations: Vec<Consultation> = storage
            .borrow()
            .iter()
            .map(|(_, consultation)| consultation.clone())
            .collect();

        if consultations.is_empty() {
            Err(Message::NotFound("No consultations found".to_string()))
        } else {
            Ok(consultations)
        }
    })
}

// Get consultations by patient id
#[ic_cdk::query]
fn get_consultations_by_patient_id(patient_id: u64) -> Result<Vec<Consultation>, Message> {
    CONSULTATIONS.with(|storage| {
        let consultations: Vec<Consultation> = storage
            .borrow()
            .iter()
            .filter(|(_, consultation)| consultation.patient_id == patient_id)
            .map(|(_, consultation)| consultation.clone())
            .collect();

        if consultations.is_empty() {
            Err(Message::NotFound("No consultations found".to_string()))
        } else {
            Ok(consultations)
        }
    })
}

// Function to create a chat
#[ic_cdk::update]
fn create_chat(payload: CreateChatPayload) -> Result<Chat, Message> {
    // Check if required fields are present
    if payload.patient_id == 0 || payload.doctor_id == 0 || payload.message.is_empty() {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Check if the patient exists
    let patient_exists =
        PATIENTS.with(|patients| patients.borrow().contains_key(&payload.patient_id));
    if !patient_exists {
        return Err(Message::NotFound("Patient not found".to_string()));
    }

    // Check if the doctor exists
    let doctor_exists = DOCTORS.with(|doctors| doctors.borrow().contains_key(&payload.doctor_id));
    if !doctor_exists {
        return Err(Message::NotFound("Doctor not found".to_string()));
    }

    // Generate a new chat ID by incrementing the ID counter
    let chat_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new Chat struct from the provided payload
    let chat = Chat {
        id: chat_id,
        patient_id: payload.patient_id,
        doctor_id: payload.doctor_id,
        message: payload.message,
        timestamp: payload.timestamp,
    };

    // Insert the new chat into the CHATS stable storage
    CHATS.with(|chats| {
        chats.borrow_mut().insert(chat_id, chat.clone());
    });

    // Return the newly created chat
    Ok(chat)
}

// Function to get a chat by id
#[ic_cdk::query]
fn get_chat_by_id(id: u64) -> Result<Chat, Message> {
    CHATS.with(|chats| match chats.borrow().get(&id) {
        Some(chat) => Ok(chat.clone()),
        None => Err(Message::NotFound("Chat not found".to_string())),
    })
}

// Function to get all chats
#[ic_cdk::query]
fn get_all_chats() -> Result<Vec<Chat>, Message> {
    CHATS.with(|storage| {
        let chats: Vec<Chat> = storage
            .borrow()
            .iter()
            .map(|(_, chat)| chat.clone())
            .collect();

        if chats.is_empty() {
            Err(Message::NotFound("No chats found".to_string()))
        } else {
            Ok(chats)
        }
    })
}

// Get chats by patient id
#[ic_cdk::query]
fn get_chats_by_patient_id(patient_id: u64) -> Result<Vec<Chat>, Message> {
    CHATS.with(|storage| {
        let chats: Vec<Chat> = storage
            .borrow()
            .iter()
            .filter(|(_, chat)| chat.patient_id == patient_id)
            .map(|(_, chat)| chat.clone())
            .collect();

        if chats.is_empty() {
            Err(Message::NotFound("No chats found".to_string()))
        } else {
            Ok(chats)
        }
    })
}

// Get chats by doctor id
#[ic_cdk::query]
fn get_chats_by_doctor_id(doctor_id: u64) -> Result<Vec<Chat>, Message> {
    CHATS.with(|storage| {
        let chats: Vec<Chat> = storage
            .borrow()
            .iter()
            .filter(|(_, chat)| chat.doctor_id == doctor_id)
            .map(|(_, chat)| chat.clone())
            .collect();

        if chats.is_empty() {
            Err(Message::NotFound("No chats found".to_string()))
        } else {
            Ok(chats)
        }
    })
}

// Function to create an appointment
#[ic_cdk::update]
fn create_appointment(payload: CreateAppointmentPayload) -> Result<Appointment, Message> {
    // Check if required fields are present
    if payload.patient_id == 0
        || payload.doctor_id == 0
        || payload.reason.is_empty()
        || payload.appointment_time == 0
    {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Check if the patient exists
    let patient_exists =
        PATIENTS.with(|patients| patients.borrow().contains_key(&payload.patient_id));
    if !patient_exists {
        return Err(Message::NotFound("Patient not found".to_string()));
    }

    // Check if the doctor exists
    let doctor_exists = DOCTORS.with(|doctors| doctors.borrow().contains_key(&payload.doctor_id));
    if !doctor_exists {
        return Err(Message::NotFound("Doctor not found".to_string()));
    }

    // Generate a new appointment ID by incrementing the ID counter
    let appointment_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new Appointment struct from the provided payload
    let appointment = Appointment {
        id: appointment_id,
        patient_id: payload.patient_id,
        doctor_id: payload.doctor_id,
        reason: payload.reason,
        appointment_time: payload.appointment_time,
        status: "scheduled".to_string(),
        video_link: None,
    };

    // Insert the new appointment into the APPOINTMENTS stable storage
    APPOINTMENTS.with(|appointments| {
        appointments
            .borrow_mut()
            .insert(appointment_id, appointment.clone());
    });

    // Return the newly created appointment
    Ok(appointment)
}

// Function to get an appointment by id
#[ic_cdk::query]
fn get_appointment_by_id(id: u64) -> Result<Appointment, Message> {
    APPOINTMENTS.with(|appointments| match appointments.borrow().get(&id) {
        Some(appointment) => Ok(appointment.clone()),
        None => Err(Message::NotFound("Appointment not found".to_string())),
    })
}

// Function to get all appointments
#[ic_cdk::query]
fn get_all_appointments() -> Result<Vec<Appointment>, Message> {
    APPOINTMENTS.with(|storage| {
        let appointments: Vec<Appointment> = storage
            .borrow()
            .iter()
            .map(|(_, appointment)| appointment.clone())
            .collect();

        if appointments.is_empty() {
            Err(Message::NotFound("No appointments found".to_string()))
        } else {
            Ok(appointments)
        }
    })
}

// Get appointments by patient id
#[ic_cdk::query]
fn get_appointments_by_patient_id(patient_id: u64) -> Result<Vec<Appointment>, Message> {
    APPOINTMENTS.with(|storage| {
        let appointments: Vec<Appointment> = storage
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.patient_id == patient_id)
            .map(|(_, appointment)| appointment.clone())
            .collect();

        if appointments.is_empty() {
            Err(Message::NotFound("No appointments found".to_string()))
        } else {
            Ok(appointments)
        }
    })
}

// Get appointments by doctor id
#[ic_cdk::query]
fn get_appointments_by_doctor_id(doctor_id: u64) -> Result<Vec<Appointment>, Message> {
    APPOINTMENTS.with(|storage| {
        let appointments: Vec<Appointment> = storage
            .borrow()
            .iter()
            .filter(|(_, appointment)| appointment.doctor_id == doctor_id)
            .map(|(_, appointment)| appointment.clone())
            .collect();

        if appointments.is_empty() {
            Err(Message::NotFound("No appointments found".to_string()))
        } else {
            Ok(appointments)
        }
    })
}

// Function to update appointment status
#[ic_cdk::update]
fn update_appointment_status(appointment_id: u64, status: String) -> Result<Appointment, Message> {
    APPOINTMENTS.with(|appointments| {
        // Get mutable access to the appointments map
        let mut appointments_map = appointments.borrow_mut();

        // Try to find and update the appointment
        if let Some(appointment) = appointments_map.get(&appointment_id) {
            // Create updated appointment with new status
            let mut updated_appointment = appointment.clone();
            updated_appointment.status = status;

            // Insert the updated appointment back into the map
            match appointments_map.insert(appointment_id, updated_appointment.clone()) {
                Some(_) => Ok(updated_appointment),
                None => Ok(updated_appointment),
            }
        } else {
            Err(Message::NotFound("Appointment not found".to_string()))
        }
    })
}

// Function to update appointment video link
#[ic_cdk::update]
fn update_appointment_video_link(
    appointment_id: u64,
    video_link: String,
) -> Result<Appointment, Message> {
    APPOINTMENTS.with(|appointments| {
        // Get mutable access to the appointments map
        let mut appointments_map = appointments.borrow_mut();

        // Try to find and update the appointment
        if let Some(appointment) = appointments_map.get(&appointment_id) {
            // Create updated appointment with new video link
            let mut updated_appointment = appointment.clone();
            updated_appointment.video_link = Some(video_link);

            // Insert the updated appointment back into the map
            match appointments_map.insert(appointment_id, updated_appointment.clone()) {
                Some(_) => Ok(updated_appointment),
                None => Ok(updated_appointment),
            }
        } else {
            Err(Message::NotFound("Appointment not found".to_string()))
        }
    })
}

// Function to create a prescription
#[ic_cdk::update]
fn create_prescription(payload: CreatePrescriptionPayload) -> Result<Prescription, Message> {
    // Check if required fields are present
    if payload.patient_id == 0 || payload.medications.is_empty() || payload.instructions.is_empty()
    {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Check if the patient exists
    let patient_exists =
        PATIENTS.with(|patients| patients.borrow().contains_key(&payload.patient_id));
    if !patient_exists {
        return Err(Message::NotFound("Patient not found".to_string()));
    }

    // Check if the doctor exists
    let doctor_exists = DOCTORS.with(|doctors| doctors.borrow().contains_key(&payload.doctor_id));
    if !doctor_exists {
        return Err(Message::NotFound("Doctor not found".to_string()));
    }

    // Generate a new prescription ID by incrementing the ID counter
    let prescription_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new Prescription struct from the provided payload
    let prescription = Prescription {
        id: prescription_id,
        patient_id: payload.patient_id,
        doctor_id: payload.doctor_id,
        medications: payload.medications,
        instructions: payload.instructions,
        issued_at: time(),
    };

    // Insert the new prescription into the PRESCRIPTIONS stable storage
    PRESCRIPTIONS.with(|prescriptions| {
        prescriptions
            .borrow_mut()
            .insert(prescription_id, prescription.clone());
    });

    // Return the newly created prescription
    Ok(prescription)
}

// Function to get a prescription by id
#[ic_cdk::query]
fn get_prescription_by_id(id: u64) -> Result<Prescription, Message> {
    PRESCRIPTIONS.with(|prescriptions| match prescriptions.borrow().get(&id) {
        Some(prescription) => Ok(prescription.clone()),
        None => Err(Message::NotFound("Prescription not found".to_string())),
    })
}

// Function to get all prescriptions
#[ic_cdk::query]
fn get_all_prescriptions() -> Result<Vec<Prescription>, Message> {
    PRESCRIPTIONS.with(|storage| {
        let prescriptions: Vec<Prescription> = storage
            .borrow()
            .iter()
            .map(|(_, prescription)| prescription.clone())
            .collect();

        if prescriptions.is_empty() {
            Err(Message::NotFound("No prescriptions found".to_string()))
        } else {
            Ok(prescriptions)
        }
    })
}

// Get prescriptions by patient id
#[ic_cdk::query]
fn get_prescriptions_by_patient_id(patient_id: u64) -> Result<Vec<Prescription>, Message> {
    PRESCRIPTIONS.with(|storage| {
        let prescriptions: Vec<Prescription> = storage
            .borrow()
            .iter()
            .filter(|(_, prescription)| prescription.patient_id == patient_id)
            .map(|(_, prescription)| prescription.clone())
            .collect();

        if prescriptions.is_empty() {
            Err(Message::NotFound("No prescriptions found".to_string()))
        } else {
            Ok(prescriptions)
        }
    })
}

// Get prescriptions by doctor id
#[ic_cdk::query]
fn get_prescriptions_by_doctor_id(doctor_id: u64) -> Result<Vec<Prescription>, Message> {
    PRESCRIPTIONS.with(|storage| {
        let prescriptions: Vec<Prescription> = storage
            .borrow()
            .iter()
            .filter(|(_, prescription)| prescription.doctor_id == doctor_id)
            .map(|(_, prescription)| prescription.clone())
            .collect();

        if prescriptions.is_empty() {
            Err(Message::NotFound("No prescriptions found".to_string()))
        } else {
            Ok(prescriptions)
        }
    })
}

// Function to create a payment
#[ic_cdk::update]
fn create_payment(payload: CreatePaymentPayload) -> Result<Payment, Message> {
    // Check if required fields are present
    if payload.patient_id == 0
        || payload.appointment_id == 0
        || payload.amount == 0
        || payload.payment_method.is_empty()
    {
        return Err(Message::InvalidPayload(
            "Missing required fields".to_string(),
        ));
    }

    // Check if the patient exists
    let patient_exists =
        PATIENTS.with(|patients| patients.borrow().contains_key(&payload.patient_id));
    if !patient_exists {
        return Err(Message::NotFound("Patient not found".to_string()));
    }

    // Check if the appointment exists
    let appointment_exists = APPOINTMENTS
        .with(|appointments| appointments.borrow().contains_key(&payload.appointment_id));
    if !appointment_exists {
        return Err(Message::NotFound("Appointment not found".to_string()));
    }

    // Generate a new payment ID by incrementing the ID counter
    let payment_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new Payment struct from the provided payload
    let payment = Payment {
        id: payment_id,
        appointment_id: payload.appointment_id,
        patient_id: payload.patient_id,
        amount: payload.amount,
        status: "pending".to_string(),
        payment_method: payload.payment_method,
    };

    // Insert the new payment into the PAYMENTS stable storage
    PAYMENTS.with(|payments| {
        payments.borrow_mut().insert(payment_id, payment.clone());
    });

    // Return the newly created payment
    Ok(payment)
}

// Function to get a payment by id
#[ic_cdk::query]
fn get_payment_by_id(id: u64) -> Result<Payment, Message> {
    PAYMENTS.with(|payments| match payments.borrow().get(&id) {
        Some(payment) => Ok(payment.clone()),
        None => Err(Message::NotFound("Payment not found".to_string())),
    })
}

// Function to get all payments
#[ic_cdk::query]
fn get_all_payments() -> Result<Vec<Payment>, Message> {
    PAYMENTS.with(|storage| {
        let payments: Vec<Payment> = storage
            .borrow()
            .iter()
            .map(|(_, payment)| payment.clone())
            .collect();

        if payments.is_empty() {
            Err(Message::NotFound("No payments found".to_string()))
        } else {
            Ok(payments)
        }
    })
}

// Get payments by patient id
#[ic_cdk::query]
fn get_payments_by_patient_id(patient_id: u64) -> Result<Vec<Payment>, Message> {
    PAYMENTS.with(|storage| {
        let payments: Vec<Payment> = storage
            .borrow()
            .iter()
            .filter(|(_, payment)| payment.patient_id == patient_id)
            .map(|(_, payment)| payment.clone())
            .collect();

        if payments.is_empty() {
            Err(Message::NotFound("No payments found".to_string()))
        } else {
            Ok(payments)
        }
    })
}

// Get payments by appointment id
#[ic_cdk::query]
fn get_payments_by_appointment_id(appointment_id: u64) -> Result<Vec<Payment>, Message> {
    PAYMENTS.with(|storage| {
        let payments: Vec<Payment> = storage
            .borrow()
            .iter()
            .filter(|(_, payment)| payment.appointment_id == appointment_id)
            .map(|(_, payment)| payment.clone())
            .collect();

        if payments.is_empty() {
            Err(Message::NotFound("No payments found".to_string()))
        } else {
            Ok(payments)
        }
    })
}

// Function to create a medical record
#[ic_cdk::update]
fn create_medical_record(payload: CreateMedicalRecordPayload) -> Result<MedicalRecord, Message> {
    // Check if required fields are present
    // if payload.patient_id == 0 || payload.doctor_id == 0 || payload.diagnosis.is_empty() || payload.treatment.is_empty() {
    //     return Err(Message::InvalidPayload(
    //         "Missing required fields".to_string(),
    //     ));
    // }

    // Check if the patient exists
    let patient_exists =
        PATIENTS.with(|patients| patients.borrow().contains_key(&payload.patient_id));
    if !patient_exists {
        return Err(Message::NotFound("Patient not found".to_string()));
    }

    // Check if the doctor exists
    let doctor_exists = DOCTORS.with(|doctors| doctors.borrow().contains_key(&payload.doctor_id));
    if !doctor_exists {
        return Err(Message::NotFound("Doctor not found".to_string()));
    }

    // Generate a new medical record ID by incrementing the ID counter
    let medical_record_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    // Create a new MedicalRecord struct from the provided payload
    let medical_record = MedicalRecord {
        id: medical_record_id,
        patient_id: payload.patient_id,
        doctor_id: payload.doctor_id,
        diagnosis: payload.diagnosis,
        treatment: payload.treatment,
        prescriptions: payload.prescriptions,
        lab_results: payload.lab_results,
        created_at: time(),
    };

    // Insert the new medical record into the MEDICAL_RECORDS stable storage
    MEDICAL_RECORDS.with(|medical_records| {
        medical_records
            .borrow_mut()
            .insert(medical_record_id, medical_record.clone());
    });

    // Return the newly created medical record
    Ok(medical_record)
}

// Function to get a medical record by id
#[ic_cdk::query]
fn get_medical_record_by_id(id: u64) -> Result<MedicalRecord, Message> {
    MEDICAL_RECORDS.with(|medical_records| match medical_records.borrow().get(&id) {
        Some(medical_record) => Ok(medical_record.clone()),
        None => Err(Message::NotFound("Medical record not found".to_string())),
    })
}

// Function to get all medical records
#[ic_cdk::query]
fn get_all_medical_records() -> Result<Vec<MedicalRecord>, Message> {
    MEDICAL_RECORDS.with(|storage| {
        let medical_records: Vec<MedicalRecord> = storage
            .borrow()
            .iter()
            .map(|(_, medical_record)| medical_record.clone())
            .collect();

        if medical_records.is_empty() {
            Err(Message::NotFound("No medical records found".to_string()))
        } else {
            Ok(medical_records)
        }
    })
}

// Get medical records by patient id
#[ic_cdk::query]
fn get_medical_records_by_patient_id(patient_id: u64) -> Result<Vec<MedicalRecord>, Message> {
    MEDICAL_RECORDS.with(|storage| {
        let medical_records: Vec<MedicalRecord> = storage
            .borrow()
            .iter()
            .filter(|(_, medical_record)| medical_record.patient_id == patient_id)
            .map(|(_, medical_record)| medical_record.clone())
            .collect();

        if medical_records.is_empty() {
            Err(Message::NotFound("No medical records found".to_string()))
        } else {
            Ok(medical_records)
        }
    })
}

// Get medical records by doctor id
#[ic_cdk::query]
fn get_medical_records_by_doctor_id(doctor_id: u64) -> Result<Vec<MedicalRecord>, Message> {
    MEDICAL_RECORDS.with(|storage| {
        let medical_records: Vec<MedicalRecord> = storage
            .borrow()
            .iter()
            .filter(|(_, medical_record)| medical_record.doctor_id == doctor_id)
            .map(|(_, medical_record)| medical_record.clone())
            .collect();

        if medical_records.is_empty() {
            Err(Message::NotFound("No medical records found".to_string()))
        } else {
            Ok(medical_records)
        }
    })
}

// Exporting the canisters to generate the candid file
ic_cdk::export_candid!();
