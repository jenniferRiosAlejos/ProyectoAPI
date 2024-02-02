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

Response3 = WS.sendRequest(findTestObject('Desarrollo ML/Cancelar Envios/Cancelación de Envíos', [('base_url') : GlobalVariable.base_url]))

print(Response3.getResponseText())

WS.verifyResponseStatusCode(Response3, 200)

// Verificar la estructura del JSON y los campos requeridos para la cancelación
def cancelJsonResponse = new groovy.json.JsonSlurper().parseText(Response3.getResponseText())

// Verificar el campo "status"
assert cancelJsonResponse.status != null : 'El campo "status" es mandatorio y no está presente en la respuesta de cancelación.'

// Verificar el campo "status_message" en caso de que "status" sea diferente de "CANCELLED"
if (cancelJsonResponse.status != 'CANCELLED') {
    assert cancelJsonResponse.status_message != null : 'El campo "status_message" es mandatorio en caso de que "status" no sea "CANCELLED" y no está presente en la respuesta de cancelación.'
}

// Verificar el campo "tracking_number" en caso de que "status" sea "CANCELLED"
if (cancelJsonResponse.status == 'CANCELLED') {
    assert cancelJsonResponse.tracking_number != null : 'El campo "tracking_number" es mandatorio en caso de que "status" sea "CANCELLED" y no está presente en la respuesta de cancelación.'
}

println('La validación de la respuesta de cancelación fue exitosa.')