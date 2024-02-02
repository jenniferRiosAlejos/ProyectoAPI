import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Response1 = WS.sendRequest(findTestObject('Desarrollo ML/Oauth/Oauth Carrier - authorizations'))

Token = WS.getElementPropertyValue(Response1, 'access_token')

print(Token)

Response2 = WS.sendRequest(findTestObject('Desarrollo ML/Crear envios/Crear Envios 200', [('base_url') : GlobalVariable.base_url]))

print(Response2.getResponseText())

WS.verifyResponseStatusCode(Response2, 200)

/// Verificar que la respuesta no sea nula
assert Response2 != null : 'La respuesta no puede ser nula.'

// Verificar la estructura del JSON y los campos requeridos
def jsonResponse = new groovy.json.JsonSlurper().parseText(Response2.getResponseText())

// Verificar el campo "message" y "id"
assert jsonResponse.message != null : 'El campo "message" es mandatorio y no está presente en la respuesta.'
assert jsonResponse.message.id != null : 'El campo "id" es mandatorio y no está presente en la respuesta.'

// Verificar el campo "status"
assert jsonResponse.message.status != null : 'El campo "status" es mandatorio y no está presente en la respuesta.'

// Verificar el campo "status_message" en caso de que "status" sea "FAILED" o "ERROR"
if (jsonResponse.message.status == 'FAILED' || jsonResponse.message.status == 'ERROR') {
    assert jsonResponse.message.status_message != null : 'El campo "status_message" es mandatorio en caso de fallo y no está presente en la respuesta.'
}

// Verificar el campo "tracking_number" en caso de que "status" sea "AUTHORIZED"
if (jsonResponse.message.status == 'AUTHORIZED') {
    assert jsonResponse.message.tracking_number != null : 'El campo "tracking_number" es mandatorio en caso de autorización exitosa y no está presente en la respuesta.'
}

// Verificar el nodo "authorization_information" en caso de que "status" sea "AUTHORIZED"
if (jsonResponse.message.status == 'AUTHORIZED') {
    assert jsonResponse.message.authorization_information != null : 'El nodo "authorization_information" es mandatorio en caso de autorización exitosa y no está presente en la respuesta.'
    
    // Verificar el campo "date" dentro de "authorization_information"
    assert jsonResponse.message.authorization_information.date != null : 'El campo "date" dentro de "authorization_information" es mandatorio en caso de autorización exitosa y no está presente en la respuesta.'
    
    // Verificar el nodo "custom_data" dentro de "authorization_information" si existe
    if (jsonResponse.message.authorization_information.custom_data != null) {
        // Verificar la presencia de campos dentro de "custom_data" según tus requerimientos
        assert jsonResponse.message.authorization_information.custom_data.key != null : 'El campo "key" dentro de "custom_data" es mandatorio y no está presente en la respuesta.'
    }
}

println('La validación de la respuesta fue exitosa.')